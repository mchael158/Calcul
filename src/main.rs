// Importa os módulos necessários do egui e eframe
use eframe::egui::{self, Color32, Context, FontFamily, FontId, Margin, Rounding, Stroke, Ui, Visuals};
use egui::CornerRadius; // Importa arredondamento de canto (embora não usado diretamente aqui)
use serde::{Serialize, Deserialize}; // Serialização para CSV e JSON
use std::fs::File; // Para salvar arquivos
use std::io::Write; // Para escrever nos arquivos
use eframe::egui::ViewportBuilder; // Para configurar o tamanho da janela

// Define a estrutura do registro mensal de produtividade
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RegistroMensal {
    mes: String,
    peso_total: f64,
    funcionarios: i32,
    dias: i32,
    horas_dia: i32,
    total_horas: i32,
    kg_hora_func: f64,
    variacao_percentual: Option<f64>,
}

// Define a estrutura do aplicativo principal
pub struct ProdutividadeApp {
    registros: Vec<RegistroMensal>, // Armazena os dados
    input_mes: String,
    input_peso: String,
    input_funcionarios: String,
    input_dias: String,
    input_horas_dia: String,
}

// Inicializa valores padrão do app (inputs vazios, lista vazia)
impl Default for ProdutividadeApp {
    fn default() -> Self {
        Self {
            registros: vec![],
            input_mes: "".to_owned(),
            input_peso: "".to_owned(),
            input_funcionarios: "".to_owned(),
            input_dias: "".to_owned(),
            input_horas_dia: "".to_owned(),
        }
    }
}

// Implementa o ciclo principal do app
impl eframe::App for ProdutividadeApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Aplica tema visual escuro e estilizado
        let mut style = (*ctx.style()).clone();
        style.visuals = Visuals {
            dark_mode: true,
            override_text_color: Some(Color32::from_rgb(100, 110, 120)), // cor padrão dos textos
            widgets: {
                let mut w = style.visuals.widgets;
                w.noninteractive.bg_fill = Color32::from_rgb(30, 30, 40);
                w.inactive.bg_fill = Color32::from_rgb(40, 40, 50);
                w.active.bg_fill = Color32::from_rgb(60, 120, 180);
                w.hovered.bg_fill = Color32::from_rgb(70, 140, 200);
                w.inactive.fg_stroke = Stroke::new(1.0, Color32::WHITE);
                w
            },
            ..style.visuals
        };

        // Define tamanhos de fonte personalizados
        style.text_styles = [
            (egui::TextStyle::Heading, FontId::new(24.0, FontFamily::Proportional)),
            (egui::TextStyle::Body, FontId::new(18.0, FontFamily::Monospace)),
            (egui::TextStyle::Button, FontId::new(18.0, FontFamily::Proportional)),
        ].into();

        ctx.set_style(style); // Aplica o estilo

        // Painel principal com cor de fundo e borda arredondada
        egui::CentralPanel::default()
            .frame(egui::Frame {
                fill: Color32::from_rgb(20, 20, 30), // fundo escuro
                inner_margin: Margin::same(20),      // margem interna
                corner_radius: Into::into(12.2),     // bordas arredondadas
                ..Default::default()
            })
            .show(ctx, |ui| {
                self.show_form(ui);           // formulário de entrada
                self.show_tabela(ui);         // tabela com registros
                self.show_export_buttons(ui); // botões para exportar dados
            });
    }
}

// Implementa as funções auxiliares da interface
impl ProdutividadeApp {
    fn show_form(&mut self, ui: &mut Ui) {
        ui.group(|ui| {
            ui.heading("📅 Adicionar Registro Mensal");
            ui.separator();
            ui.add_space(10.0);

            // Campos de entrada do formulário
            ui.horizontal(|ui| {
                ui.label("Mês:");
                ui.text_edit_singleline(&mut self.input_mes);
            });
            ui.horizontal(|ui| {
                ui.label("Peso total (kg):");
                ui.text_edit_singleline(&mut self.input_peso);
            });
            ui.horizontal(|ui| {
                ui.label("Funcionários:");
                ui.text_edit_singleline(&mut self.input_funcionarios);
            });
            ui.horizontal(|ui| {
                ui.label("Dias trabalhados:");
                ui.text_edit_singleline(&mut self.input_dias);
            });
            ui.horizontal(|ui| {
                ui.label("Horas por dia:");
                ui.text_edit_singleline(&mut self.input_horas_dia);
            });

            ui.add_space(10.0);

            // Botão para adicionar novo registro mensal
            if ui.add(
                egui::Button::new("➕ Adicionar Mês")
                    .fill(Color32::from_rgb(0, 150, 100)) // cor do botão
                    .corner_radius(8)
                    .min_size(egui::vec2(180.0, 40.0))
            ).clicked() {
                // Tenta converter os inputs
                if let (Ok(peso), Ok(func), Ok(dias), Ok(horas)) = (
                    self.input_peso.parse::<f64>(),
                    self.input_funcionarios.parse::<i32>(),
                    self.input_dias.parse::<i32>(),
                    self.input_horas_dia.parse::<i32>(),
                ) {
                    // Calcula produtividade
                    let total_horas = func * dias * horas;
                    let kg_hora_func = peso / total_horas as f64;

                    // Calcula variação percentual se houver histórico
                    let variacao = self.registros.last().map(|r| {
                        ((kg_hora_func - r.kg_hora_func) / r.kg_hora_func) * 100.0
                    });

                    // Adiciona novo registro
                    self.registros.push(RegistroMensal {
                        mes: self.input_mes.clone(),
                        peso_total: peso,
                        funcionarios: func,
                        dias,
                        horas_dia: horas,
                        total_horas,
                        kg_hora_func,
                        variacao_percentual: variacao,
                    });

                    // Limpa os campos após adicionar
                    self.input_mes.clear();
                    self.input_peso.clear();
                    self.input_funcionarios.clear();
                    self.input_dias.clear();
                    self.input_horas_dia.clear();
                }
            }
        });
    }

    fn show_tabela(&mut self, ui: &mut Ui) {
        ui.add_space(20.0);
        ui.separator();

        // Título e botão de apagar tabela
        ui.horizontal(|ui| {
            ui.heading("📊 Registros Mensais");
            if ui.add(
                egui::Button::new("apagar tabela")
                    .fill(Color32::from_rgb(80, 70, 23))
                    .corner_radius(7)
            ).clicked() {
                self.registros.clear(); // limpa todos os registros
            }
        });

        ui.separator();

        // Área com rolagem contendo a tabela de dados
        egui::ScrollArea::vertical().show(ui, |ui| {
            egui::Grid::new("tabela_produtividade")
                .striped(true)
                .spacing([15.0, 8.0])
                .show(ui, |ui| {
                    // Cabeçalho da tabela
                    ui.label("Mês");
                    ui.label("Peso");
                    ui.label("Funcionários");
                    ui.label("Dias");
                    ui.label("Horas/dia");
                    ui.label("Total Horas");
                    ui.label("kg/H/Func");
                    ui.label("Variação %");
                    ui.end_row();

                    // Linhas com os dados
                    for r in &self.registros {
                        ui.label(&r.mes);
                        ui.label(format!("{:.2}", r.peso_total));
                        ui.label(r.funcionarios.to_string());
                        ui.label(r.dias.to_string());
                        ui.label(r.horas_dia.to_string());
                        ui.label(r.total_horas.to_string());
                        ui.label(format!("{:.2}", r.kg_hora_func));
                        ui.label(match r.variacao_percentual {
                            Some(v) => format!("{:.2}%", v),
                            None => "N/A".into(),
                        });
                        ui.end_row();
                    }
                });
        });
    }

    fn show_export_buttons(&self, ui: &mut Ui) {
        ui.add_space(20.0);
        ui.separator();

        // Botões horizontais para exportar CSV e JSON
        ui.horizontal(|ui| {
            // Exporta como CSV
            if ui.add(
                egui::Button::new("💾 Exportar CSV")
                    .fill(Color32::from_rgb(60, 100, 200))
                    .rounding(Rounding::same(8))
                    .min_size(egui::vec2(160.0, 40.0))
            ).clicked() {
                if let Ok(mut wtr) = csv::Writer::from_path("produtividade.csv") {
                    for r in &self.registros {
                        let _ = wtr.serialize(r);
                    }
                    let _ = wtr.flush();
                }
            }

            // Exporta como JSON
            if ui.add(
                egui::Button::new("📄 Exportar JSON")
                    .fill(Color32::from_rgb(100, 200, 100))
                    .rounding(Rounding::same(8))
                    .min_size(egui::vec2(160.0, 40.0))
            ).clicked() {
                if let Ok(json) = serde_json::to_string_pretty(&self.registros) {
                    if let Ok(mut file) = File::create("produtividade.json") {
                        let _ = file.write_all(json.as_bytes());
                    }
                }
            }
        });
    }
}

// Função principal que inicia o app com tamanho personalizado
fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([900.0, 700.0]), // tamanho inicial da janela
        ..Default::default()
    };

    let _ = eframe::run_native(
        "📈 Produtividade App", // título da janela
        options,
        Box::new(|_cc| Ok(Box::new(ProdutividadeApp::default()))), // instancia o app
    );
}

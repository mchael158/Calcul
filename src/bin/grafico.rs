use eframe::egui;
use eframe::egui::{Color32, Context, FontFamily, FontId, Visuals};
use egui_plot::{Line, Plot, PlotPoints};
use serde::Deserialize;
use std::fs;

#[derive(Debug, Clone, Deserialize)]
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

pub struct GraficoApp {
    registros: Vec<RegistroMensal>,
}

impl Default for GraficoApp {
    fn default() -> Self {
        let dados_json = fs::read_to_string("produtividade.json").unwrap_or_else(|_| "[]".into());
        let registros: Vec<RegistroMensal> = serde_json::from_str(&dados_json).unwrap_or_default();
        Self { registros }
    }
}

impl eframe::App for GraficoApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        // Estilo escuro e tipografia
        let mut style = (*ctx.style()).clone();
        style.visuals = Visuals::dark();
        style.text_styles = [
            (egui::TextStyle::Heading, FontId::new(24.0, FontFamily::Proportional)),
            (egui::TextStyle::Body, FontId::new(18.0, FontFamily::Monospace)),
        ]
        .into();
        ctx.set_style(style);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("📈 Gráfico de Produtividade");

            if self.registros.is_empty() {
                ui.label("Nenhum dado disponível. Exporte o JSON no outro app primeiro.");
                return;
            }

            // Prepara os pontos para o gráfico de linha
            let pontos: PlotPoints = self
                .registros
                .iter()
                .enumerate()
                .map(|(i, r)| [i as f64, r.kg_hora_func])
                .collect();

            let linha = Line::new("kg/h/func", pontos);


            Plot::new("grafico_produtividade")
                .legend(egui_plot::Legend::default())
                .height(400.0)
                .show(ui, |plot_ui| {
                    plot_ui.line(linha);
                });

            // Lista os meses abaixo do gráfico
            ui.separator();
            ui.label("Meses:");
            for r in &self.registros {
                ui.label(format!("📅 {}", r.mes));
            }
        });
    }
}

fn main() {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([800.0, 600.0]),
        ..Default::default()
    };

    let _ = eframe::run_native(
        "📊 Visualizador de Gráficos",
        options,
        Box::new(|_cc| Ok(Box::new(GraficoApp::default()))),
    );
}

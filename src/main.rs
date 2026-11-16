use macroquad::prelude::*;

struct Jogador {
    x: f32,
    y: f32,
    largura: f32,
    altura: f32,
    vel_x: f32,
    vel_y: f32,
    gravidade_invertida: bool,
    no_chao: bool,
}

#[macroquad::main("VVVVVV Rust Android")]
async fn main() {
    let mut player = Jogador {
        x: 200.0,
        y: 200.0,
        largura: 30.0,
        altura: 50.0,
        vel_x: 0.0,
        vel_y: 0.0,
        gravidade_invertida: false,
        no_chao: false,
    };

    let mut botao_acao_pressionado_antes = false;

    loop {
        let dt = get_frame_time();
        let w = screen_width();
        let h = screen_height();

        // --- 1. DEFINIÇÃO DO GAMEPAD VIRTUAL NA TELA ---
        let btn_esq = Rect::new(30.0, h - 110.0, 80.0, 80.0);
        let btn_dir = Rect::new(130.0, h - 110.0, 80.0, 80.0);
        let btn_acao = Rect::new(w - 120.0, h - 110.0, 90.0, 90.0);

        let mut mover_esq = is_key_down(KeyCode::Left) || is_key_down(KeyCode::A);
        let mut mover_dir = is_key_down(KeyCode::Right) || is_key_down(KeyCode::D);
        let mut acao_agora = is_key_down(KeyCode::Space) || is_key_down(KeyCode::Z);

        // Leitura Multi-Touch para celular
        for touch in touches() {
            let pos = touch.position;
            if btn_esq.contains(pos) {
                mover_esq = true;
            }
            if btn_dir.contains(pos) {
                mover_dir = true;
            }
            if btn_acao.contains(pos) {
                acao_agora = true;
            }
        }

        // --- 2. FÍSICA E CONTROLE ---
        let velocidade = 320.0;
        player.vel_x = 0.0;
        if mover_esq {
            player.vel_x = -velocidade;
        }
        if mover_dir {
            player.vel_x = velocidade;
        }

        // Inverte a gravidade apenas no momento do clique
        if acao_agora && !botao_acao_pressionado_antes && player.no_chao {
            player.gravidade_invertida = !player.gravidade_invertida;
            player.no_chao = false;
        }
        botao_acao_pressionado_antes = acao_agora;

        // Aplica gravidade (para baixo ou para cima)
        let forca_gravidade = if player.gravidade_invertida { -1400.0 } else { 1400.0 };
        player.vel_y += forca_gravidade * dt;

        // Atualiza posição
        player.x += player.vel_x * dt;
        player.y += player.vel_y * dt;

        // --- 3. COLISÃO SIMPLES COM CHÃO E TETO ---
        let teto = 40.0;
        let chao = h - 140.0;

        player.no_chao = false;

        if player.y + player.altura >= chao {
            player.y = chao - player.altura;
            player.vel_y = 0.0;
            if !player.gravidade_invertida {
                player.no_chao = true;
            }
        } else if player.y <= teto {
            player.y = teto;
            player.vel_y = 0.0;
            if player.gravidade_invertida {
                player.no_chao = true;
            }
        }

        // Limites laterais
        if player.x < 20.0 { player.x = 20.0; }
        if player.x + player.largura > w - 20.0 { player.x = w - 20.0 - player.largura; }

        // --- 4. DESENHO NA TELA ---
        clear_background(Color::from_rgba(15, 15, 25, 255));

        // Paredes da sala
        draw_rectangle(0.0, 0.0, w, teto, SKYBLUE);
        draw_rectangle(0.0, chao, w, 20.0, SKYBLUE);

        // Jogador (Amarelo normal, Magenta invertido)
        let cor_player = if player.gravidade_invertida { MAGENTA } else { YELLOW };
        draw_rectangle(player.x, player.y, player.largura, player.altura, cor_player);

        // Desenha o Gamepad Virtual (HUD)
        let cor_btn = Color::from_rgba(255, 255, 255, 70);
        let cor_btn_ativo = Color::from_rgba(255, 255, 255, 150);

        draw_rectangle(btn_esq.x, btn_esq.y, btn_esq.w, btn_esq.h, if mover_esq { cor_btn_ativo } else { cor_btn });
        draw_text("<", btn_esq.x + 30.0, btn_esq.y + 50.0, 40.0, WHITE);

        draw_rectangle(btn_dir.x, btn_dir.y, btn_dir.w, btn_dir.h, if mover_dir { cor_btn_ativo } else { cor_btn });
        draw_text(">", btn_dir.x + 30.0, btn_dir.y + 50.0, 40.0, WHITE);

        draw_rectangle(btn_acao.x, btn_acao.y, btn_acao.w, btn_acao.h, if acao_agora { cor_btn_ativo } else { cor_btn });
        draw_text("FLIP", btn_acao.x + 15.0, btn_acao.y + 52.0, 28.0, WHITE);

        next_frame().await
    }
}

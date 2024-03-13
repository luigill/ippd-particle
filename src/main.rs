use rand::Rng;
use rayon::prelude::*;

struct Particula {
    id: usize,
    posicao: (f32, f32),
    velocidade: (f32, f32),
}

struct SistemaParticulas {
    particulas: Vec<Particula>,
    contador: usize,
}

impl SistemaParticulas {
    fn new() -> Self {
        SistemaParticulas {
            particulas: Vec::new(),
            contador: 0,
        }
    }

    fn adiciona_particula(&mut self, posicao: (f32, f32), velocidade: (f32, f32)) {
        let id = self.contador;
        self.contador += 1;

        self.particulas.push(Particula {
            id,
            posicao,
            velocidade,
        });
    }

    fn update(&mut self) {
        self.particulas.par_iter_mut().for_each(|particle| {
            let antiga_posicao = particle.posicao;

            let mut rng = rand::thread_rng();
            particle.velocidade.0 += rng.gen_range(-0.2..0.2);
            particle.velocidade.1 += rng.gen_range(-0.2..0.2);

            particle.posicao.0 += particle.velocidade.0;
            particle.posicao.1 += particle.velocidade.1;

            println!(
                "Particula {} se moveu de {:?} para {:?}",
                particle.id, antiga_posicao, particle.posicao
            );
        });
    }
}

fn main() {
    let mut sistema = SistemaParticulas::new();

    let mut rng = rand::thread_rng();

    for _ in 0..15 {
        let posicao = (rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0));
        let velocidade = (rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0));
        sistema.adiciona_particula(posicao, velocidade);
    }

    for _ in 0..100 {
        sistema.update();
        print!("--------\n");
    }
}

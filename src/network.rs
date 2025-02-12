use std::{  fs::File, io::{Read, Write},};
use  serde::{Deserialize, Serialize};
use serde_json::{from_str, json};

use  super::{activations::Activation, matrix::Matrix};


pub struct Network<'a> {
	layers: Vec<usize>,
	poids : Vec<Matrix>,
	biais: Vec<Matrix>,
	data: Vec<Matrix>,
	learning_rate: f64,
	activation: Activation<'a>,
}

#[derive(Serialize, Deserialize)]
struct SaveData { 
    poids : Vec<Vec<Vec<f64>>>,
    biais : Vec<Vec<Vec<f64>>>,
}

impl Network<'_> {

    pub fn  new<'a> (
        layers : Vec<usize>,
        learning_rate : f64,
        activation : Activation<'a>,
    ) -> Network<'a> {
        let mut poids = vec![];
        let mut  biais = vec![];

        for i in 0..layers.len() -1 {
            poids.push(Matrix::random(layers[i + 1], layers[i]));
            biais.push(Matrix::random(layers[i + 1], 1));
        }

        Network {
			layers,
			poids,
			biais,
			data: vec![],
			learning_rate,
			activation,
		}
    } 

    pub fn feed_forward(&mut self, inputs: Vec<f64>)  -> Vec<f64> {
        if  inputs.len() != self.layers[0] {
            panic!("invalide entrez")
        }

        let mut  current = Matrix::from(vec![inputs]).transpose();
        self.data = vec![current.clone()];

        for  i in 0..self.layers.len() -1 {
            current = self.poids[i]
                .multiply(&current)
                .add(&self.biais[i])
                .map(self.activation.fonction);
            self.data.push(current.clone());
        }

		current.transpose().data[0].to_owned()
    }

    pub fn back_propogate(&mut self, outputs: Vec<f64>, targets: Vec<f64>) {
		if targets.len() != self.layers[self.layers.len() - 1] {
			panic!("Invalid targets length");
		}

		let parsed = Matrix::from(vec![outputs]).transpose();
		let mut errors = Matrix::from(vec![targets]).transpose().subtract(&parsed);
		let mut gradients = parsed.map(self.activation.derivation);

		for i in (0..self.layers.len() - 1).rev() {
			gradients = gradients
				.dot_multiply(&errors)
				.map(&|x| x * self.learning_rate);

			self.poids[i] = self.poids[i].add(&gradients.multiply(&self.data[i].transpose()));
			self.biais[i] = self.biais[i].add(&gradients);

			errors = self.poids[i].transpose().multiply(&errors);
			gradients = self.data[i].map(self.activation.derivation);
		}
	}


	pub fn train(&mut self, inputs: Vec<Vec<f64>>, targets: Vec<Vec<f64>>, epochs: u128) {
		for i in 1..=epochs {
			if epochs < 100 || i % (epochs / 100) == 0 {
				println!("Epoch {} of {}", i, epochs);
			}

			for j in 0..inputs.len() {
				let outputs = self.feed_forward(inputs[j].clone());
				self.back_propogate(outputs, targets[j].clone());
			}
		}
	}


	pub fn save(&self, file: String) {
		let mut file = File::create(file).expect("Unable to touch save file");

		file.write_all(
			json!({
				"poids": self.poids.clone().into_iter().map(|matrix| matrix.data).collect::<Vec<Vec<Vec<f64>>>>(),
				"biais": self.biais.clone().into_iter().map(|matrix| matrix.data).collect::<Vec<Vec<Vec<f64>>>>()
			}).to_string().as_bytes(),
		).expect("Unable to write to save file");
	}

    pub fn load(&mut self, file: String) {
		let mut file = File::open(file).expect("Unable to open save file");
		let mut buffer = String::new();

		file.read_to_string(&mut buffer)
			.expect("Unable to read save file");

		let save_data: SaveData = from_str(&buffer).expect("Unable to serialize save data");

		let mut poids = vec![];
		let mut biais = vec![];

		for i in 0..self.layers.len() - 1 {
			poids.push(Matrix::from(save_data.poids[i].clone()));
			biais.push(Matrix::from(save_data.biais[i].clone()));
		}

		self.poids = poids;
		self.biais = biais;
	}
}


#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Env, String, Symbol, Vec,
};

// Struktur data observasi spesies
#[contracttype]
#[derive(Clone, Debug)]
pub struct SpeciesObservation {
    id: u64,
    species_name: String,
    location: String,
    description: String,
    observer: String,
}

// Key storage
const OBSERVATION_DATA: Symbol = symbol_short!("BIOCHAIN");

#[contract]
pub struct BioChainContract;

#[contractimpl]
impl BioChainContract {

    // Ambil semua data observasi
    pub fn get_observations(env: Env) -> Vec<SpeciesObservation> {
        return env
            .storage()
            .instance()
            .get(&OBSERVATION_DATA)
            .unwrap_or(Vec::new(&env));
    }

    // Tambah observasi spesies baru
    pub fn add_observation(
        env: Env,
        species_name: String,
        location: String,
        description: String,
        observer: String,
    ) -> String {

        // Ambil data lama
        let mut observations: Vec<SpeciesObservation> = env
            .storage()
            .instance()
            .get(&OBSERVATION_DATA)
            .unwrap_or(Vec::new(&env));

        // Buat observasi baru
        let observation = SpeciesObservation {
            id: env.prng().gen::<u64>(),
            species_name,
            location,
            description,
            observer,
        };

        // Tambahkan ke vector
        observations.push_back(observation);

        // Simpan kembali
        env.storage()
            .instance()
            .set(&OBSERVATION_DATA, &observations);

        return String::from_str(
            &env,
            "Observasi spesies berhasil ditambahkan"
        );
    }

    // Hapus observasi berdasarkan id
    pub fn delete_observation(env: Env, id: u64) -> String {

        let mut observations: Vec<SpeciesObservation> = env
            .storage()
            .instance()
            .get(&OBSERVATION_DATA)
            .unwrap_or(Vec::new(&env));

        // Cari data
        for i in 0..observations.len() {

            if observations.get(i).unwrap().id == id {

                observations.remove(i);

                env.storage()
                    .instance()
                    .set(&OBSERVATION_DATA, &observations);

                return String::from_str(
                    &env,
                    "Observasi berhasil dihapus"
                );
            }
        }

        return String::from_str(
            &env,
            "Observasi tidak ditemukan"
        );
    }
}

mod test;
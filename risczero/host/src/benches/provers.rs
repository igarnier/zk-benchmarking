use risc0_zkvm::prove::Prover;
use std::rc::Rc;

#[derive(Clone, PartialEq, Copy)]
pub enum Name {
    CpuSHA256,
    CpuPoseidon,
    Bonsai,
}

impl Name {
    pub fn to_string(&self) -> String {
        match self {
            Self::CpuSHA256 => String::from("CpuSHA256"),
            Self::CpuPoseidon => String::from("CpuPoseidon"),
            Self::Bonsai => String::from("Bonsai"),
        }
    }

    pub fn get_prover(&self) -> Rc<dyn Prover> {
        match self {
            Self::CpuSHA256 => risc0_zkvm::prove::get_prover("cpu"),
            Self::CpuPoseidon => risc0_zkvm::prove::get_prover("cpu:poseidon"),
            Self::Bonsai => risc0_zkvm::prove::get_prover("$bonsai"),
        }
    }
}

// pub const DEFAULT: Name = Name::CpuSHA256;

pub const PROVERS: [Name; 3] = [Name::Bonsai, Name::CpuSHA256, Name::CpuPoseidon];

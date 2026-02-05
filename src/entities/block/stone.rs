use crate::{
    entities::traits::properties::PhysicalProperties,
    math::units::{Mass, Pressure, Temperature, Volume},
};

use super::super::traits::{properties, Entity};

#[derive(Debug, Clone, Copy)]
pub struct Stone {
    temperature: Temperature,
    pressure: Pressure,
    mass: Mass,
    volume: Volume,
    solid_fraction: f32,
    water_fraction: f32,
    gas_fraction: f32,
}

impl Entity for Stone { }

impl properties::PhysicalProperties for Stone {
    fn temperature(&self) -> crate::math::units::Temperature {
        self.temperature
    }

    fn pressure(&self) -> crate::math::units::Pressure {
        self.pressure
    }

    fn mass(&self) -> Mass {
        self.mass
    }

    fn volume(&self) -> crate::math::units::Volume {
        self.volume
    }

    fn get_mut_temperature(&mut self) -> &mut crate::math::units::Temperature {
        &mut self.temperature
    }

    fn get_mut_pressure(&mut self) -> &mut crate::math::units::Pressure {
        &mut self.pressure
    }

    fn get_mut_mass(&mut self) -> &mut Mass {
        &mut self.mass
    }

    fn get_mut_volume(&mut self) -> &mut crate::math::units::Volume {
        &mut self.volume
    }
}

impl properties::ChemicalProperties for Stone {}

impl properties::Composition for Stone {
    fn water_fraction(&self) -> f32 {
        self.water_fraction
    }

    fn solid_fraction(&self) -> f32 {
        self.solid_fraction
    }

    fn gas_fraction(&self) -> f32 {
        self.gas_fraction
    }

    fn total_mass(&self) -> crate::math::units::Mass {
        self.mass()
    }
}

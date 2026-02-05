use crate::math::units::{Density, Mass, Pressure, Temperature, Volume};

pub trait PhysicalProperties {
    /// Celsius
    fn temperature(&self) -> Temperature;

    fn pressure(&self) -> Pressure;

    /// kg
    fn mass(&self) -> Mass;

    /// m^3
    fn volume(&self) -> Volume;

    fn density(&self) -> Result<Density, &'static str> {
        let volume = self.volume();
        if volume.0 <= 0.0 {
            return Err("Cannot calculate density: volume is zero or negative");
        }

        Ok(self.mass() / volume)
    }

    /// Celsius
    fn get_mut_temperature(&mut self) -> &mut Temperature;

    fn get_mut_pressure(&mut self) -> &mut Pressure;

    /// kg
    fn get_mut_mass(&mut self) -> &mut Mass;

    /// m^3
    fn get_mut_volume(&mut self) -> &mut Volume;

    fn set_temperature(&mut self, temp: Temperature) -> Result<(), &'static str> {
        if temp < -273.15 {
            return Err("Temperature below absolute zero");
        }
        *self.get_mut_temperature() = temp;

        Ok(())
    }

    fn set_mass(&mut self, mass: Mass) -> Result<(), &'static str> {
        if mass <= 0.0 {
            return Err("Mass must be positive");
        }
        *self.get_mut_mass() = mass;

        Ok(())
    }

    fn set_pressure(&mut self, pressure: Pressure) -> Result<(), &'static str> {
        *self.get_mut_pressure() = pressure;

        Ok(())
    }

    fn set_volume(&mut self, volume: Volume) -> Result<(), &'static str> {
        if volume <= Volume(0.0) {
            return Err("Volume must be positive");
        }
        *self.get_mut_volume() = volume;

        Ok(())
    }

    fn debug_info(&self) -> String {
        format!(
            "temperature: {:?}; mass: {:?}; volume: {:?}; density: {:?}; pressure: {:?};",
            self.temperature(),
            self.mass(),
            self.volume(),
            self.density(),
            self.pressure()
        )
    }
}

pub trait ChemicalProperties: Composition {
    fn water_mass(&self) -> Mass {
        self.total_mass() * self.water_fraction()
    }

    fn debug_info(&self) -> String {
        format!(
            "Water mass: {:?}; Composition: [{}]",
            self.water_mass(),
            Composition::debug_info(self)
        )
    }
}

pub trait Composition {
    fn water_fraction(&self) -> f32; // 0..1
    fn solid_fraction(&self) -> f32; // 0..1
    fn gas_fraction(&self) -> f32; // 0..1

    fn total_mass(&self) -> Mass; // kg

    fn validate_composition(&self) -> Result<(), &'static str> {
        let sum = self.water_fraction() + self.solid_fraction() + self.gas_fraction();
        if (sum - 1.0).abs() > 1e-8 {
            return Err("Composition fractions must sum to 1.0");
        }

        Ok(())
    }

    fn debug_info(&self) -> String {
        format!(
            "Water fraction: {}; Solid fraction: {}; Gas fraction: {}; Total mass: {:?}",
            self.water_fraction(),
            self.solid_fraction(),
            self.gas_fraction(),
            self.total_mass()
        )
    }
}

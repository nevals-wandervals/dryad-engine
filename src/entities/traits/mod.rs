//! A description of the common properties of all entities.

pub mod properties;

pub trait Entity : self::properties::ChemicalProperties + self::properties::PhysicalProperties { }


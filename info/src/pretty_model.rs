use l6t::model;
use crate::pretty::*;

impl Pretty for model::L6Patch {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result {
        writeln!(pp, "File type: L6T")?;
        writeln!(pp)?;
        Pretty::fmt(&self.meta, pp)?;
        Pretty::fmt(&self.target_device, pp)?;
        Pretty::fmt(&self.models, pp)?;
        Ok(())
    }
}

impl Pretty for model::MetaTags {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result {

        let str_field = |name: &str, field: &str, pp: &mut PrettyPrinter| {
            if !field.is_empty() {
                writeln!(pp, "{:16}: {}", name, field)
            } else {
                Ok(())
            }
        };
        let date_field = |name: &str, field: &usize, pp: &mut PrettyPrinter| {
            if *field != 0 {
                writeln!(pp, "{:16}: {}", name, field)
            } else {
                Ok(())
            }
        };

        writeln!(pp, "Info:")?;
        pp.indent += 1;
        str_field("author", &self.author, pp)?;
        str_field("guitarist", &self.guitarist, pp)?;
        str_field("band", &self.band, pp)?;
        str_field("song", &self.song, pp)?;
        str_field("style", &self.style, pp)?;
        str_field("pickup style", &self.pickup_style, pp)?;
        str_field("pickup position", &self.pickup_position, pp)?;
        date_field("date", &self.date, pp)?;
        str_field("amp name", &self.amp_name, pp)?;
        str_field("creator app", &self.creator_app, pp)?;
        str_field("creator app ver", &self.creator_app_version, pp)?;
        str_field("comments", &self.comments, pp)?;
        pp.indent -= 1;
        Ok(())
    }
}

impl Pretty for model::TargetDevice {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result {
        writeln!(pp, "Device:")?;
        pp.indent += 1;
        writeln!(pp, "{:16}: {:#08x}", "id", self.midi_id)?;
        writeln!(pp, "{:16}: {}", "name", self.name)?;
        writeln!(pp, "{:16}: {}", "version", self.version)?;
        pp.indent -= 1;
        Ok(())
    }
}

impl Pretty for Vec<model::Model> {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result {
        writeln!(pp, "Model:")?;
        pp.indent += 1;

        for v in self.iter() {
            Pretty::fmt(v, pp)?;
        }
        pp.indent -= 1;
        Ok(())
    }
}

impl Pretty for model::Model {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result {
        writeln!(pp, "[{:#04x}] model={:#08x} ordinal={} {}",
                 self.slot_id, self.model_id, self.ordinal,
                 if self.enabled { "enabled" } else { "disabled" } )?;
        pp.indent += 1;
        for param in self.params.iter() {
            Pretty::fmt(param, pp)?
        }
        pp.indent -= 1;
        Ok(())
    }
}

impl Pretty for model::ModelParam {
    fn fmt(&self, pp: &mut PrettyPrinter) -> fmt::Result {
        writeln!(pp, "id={:#08x} {}", self.param_id, self.value)
    }
}

use crate::utils::type_description::print_type_description;
use crate::utils::{print_first_paragraph_with_indent, with_indent};
use clap::Args;

use std::fmt::Write;
use std::write;

use color_eyre::eyre::eyre;
use frame_metadata::v15::PalletMetadata;

use scale_info::form::PortableForm;

use subxt::Metadata;

#[derive(Debug, Clone, Args)]
pub struct ConstantsSubcommand {
    constant: Option<String>,
}

pub(crate) fn explore_constants(
    command: ConstantsSubcommand,
    metadata: &Metadata,
    pallet_metadata: &PalletMetadata<PortableForm>,
) -> color_eyre::Result<String> {
    let pallet_name = pallet_metadata.name.as_str();
    let Some(constant_name) = command.constant else {
        let available_constants = print_available_constants(pallet_metadata, pallet_name);
        let mut output = "Usage:".to_string();
        writeln!(output, "    subxt explore {pallet_name} constants <CONSTANT>")?;
        writeln!(output, "        explore a specific call within this pallet\n\n{available_constants}")?;
        return Ok(output);
    };

    // if specified constant is wrong, show user the constants to choose from (but this time as an error):
    let Some(constant) = pallet_metadata.constants.iter().find(|constant| constant.name.to_lowercase() == constant_name.to_lowercase())   else {
        let available_constants = print_available_constants(pallet_metadata, pallet_name);
        let mut description = "Usage:".to_string();
        writeln!(description, "    subxt explore {pallet_name} constants <CONSTANT>")?;
        writeln!(description, "        explore a specific call within this pallet\n\n{available_constants}")?;
        let err = eyre!("constant \"{constant_name}\" not found in \"{pallet_name}\" pallet!\n\n{description}");
        return Err(err);
    };

    // docs
    let mut output = String::new();
    let doc_string = print_first_paragraph_with_indent(&constant.docs, 4);
    if !doc_string.is_empty() {
        write!(output, "Description:\n{doc_string}")?;
    }

    // shape
    let mut type_description = print_type_description(&constant.ty.id, metadata.types())?;
    type_description = with_indent(type_description, 4);
    write!(
        output,
        "\n\nThe constant has the following shape:\n{type_description}"
    )?;

    // value
    let scale_val = scale_value::scale::decode_as_type(
        &mut &constant.value[..],
        constant.ty.id,
        metadata.types(),
    )?;
    write!(
        output,
        "\n\nThe value of the constant is:\n    {}",
        scale_value::stringify::to_string(&scale_val)
    )?;
    Ok(output)
}

fn print_available_constants(
    pallet_metadata: &PalletMetadata<PortableForm>,
    pallet_name: &str,
) -> String {
    if pallet_metadata.constants.is_empty() {
        return format!("No <CONSTANT>'s available in the \"{pallet_name}\" pallet.");
    }
    let mut output = format!("Available <CONSTANT>'s in the \"{pallet_name}\" pallet:");
    let mut strings: Vec<_> = pallet_metadata.constants.iter().map(|c| &c.name).collect();
    strings.sort();
    for constant in strings {
        output.push_str("\n    ");
        output.push_str(constant);
    }
    output
}

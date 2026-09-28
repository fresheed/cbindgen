/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

use std::collections::HashMap;
use std::mem;

use crate::bindgen::ir::{
    Enum, Field, GenericArgument, GenericPath, Item, OpaqueItem, Path, Struct, Typedef, Union,
    VariantBody,
};
use crate::bindgen::library::Library;

#[derive(Default, Clone, Debug)]
pub struct Monomorphs {
    replacements: HashMap<GenericPath, Path>,
    opaques: Vec<OpaqueItem>,
    typedefs: Vec<Typedef>,
    // Pairs of (generic item, its monomorph).
    // This is needed for subsequent check for zero-sized fields
    structs: Vec<(Struct, Struct)>,
    unions: Vec<(Union, Union)>,
    enums: Vec<(Enum, Enum)>,
}

impl Monomorphs {
    pub fn contains(&self, path: &GenericPath) -> bool {
        self.replacements.contains_key(path)
    }

    pub fn insert_struct(
        &mut self,
        library: &Library,
        generic: &Struct,
        monomorph: Struct,
        arguments: Vec<GenericArgument>,
    ) {
        let replacement_path = GenericPath::new(generic.path.clone(), arguments);

        debug_assert!(generic.is_generic());
        debug_assert!(!self.contains(&replacement_path));

        self.replacements
            .insert(replacement_path, monomorph.path.clone());

        monomorph.add_monomorphs(library, self);

        self.structs.push((generic.clone(), monomorph));
    }

    pub fn insert_enum(
        &mut self,
        library: &Library,
        generic: &Enum,
        monomorph: Enum,
        arguments: Vec<GenericArgument>,
    ) {
        let replacement_path = GenericPath::new(generic.path.clone(), arguments);

        debug_assert!(generic.is_generic());
        debug_assert!(!self.contains(&replacement_path));

        self.replacements
            .insert(replacement_path, monomorph.path.clone());

        monomorph.add_monomorphs(library, self);

        self.enums.push((generic.clone(), monomorph));
    }

    pub fn insert_union(
        &mut self,
        library: &Library,
        generic: &Union,
        monomorph: Union,
        arguments: Vec<GenericArgument>,
    ) {
        let replacement_path = GenericPath::new(generic.path.clone(), arguments);

        debug_assert!(generic.is_generic());
        debug_assert!(!self.contains(&replacement_path));

        self.replacements
            .insert(replacement_path, monomorph.path.clone());

        monomorph.add_monomorphs(library, self);

        self.unions.push((generic.clone(), monomorph));
    }

    pub fn insert_opaque(
        &mut self,
        generic: &OpaqueItem,
        monomorph: OpaqueItem,
        arguments: Vec<GenericArgument>,
    ) {
        let replacement_path = GenericPath::new(generic.path.clone(), arguments);

        debug_assert!(generic.is_generic());
        debug_assert!(!self.contains(&replacement_path));

        self.replacements
            .insert(replacement_path, monomorph.path.clone());
        self.opaques.push(monomorph);
    }

    pub fn insert_typedef(
        &mut self,
        library: &Library,
        generic: &Typedef,
        monomorph: Typedef,
        arguments: Vec<GenericArgument>,
    ) {
        let replacement_path = GenericPath::new(generic.path.clone(), arguments);

        debug_assert!(generic.is_generic());
        debug_assert!(!self.contains(&replacement_path));

        self.replacements
            .insert(replacement_path, monomorph.path.clone());

        monomorph.add_monomorphs(library, self);

        self.typedefs.push(monomorph);
    }

    /// C++ bindings keep generic items as templates and write zero-sized generic
    /// arguments as `void`. Instantiations that lose fields to such arguments
    /// can't be represented correctly, so warn about them.
    pub fn warn_zst_instantiations(&self) {
        fn warn_missing(
            generic_name: &str,
            monomorph_name: &str,
            generic: &[Field],
            monomorph: &[Field],
        ) {
            let missing: Vec<&str> = generic
                .iter()
                .map(|f| f.name.as_str())
                .filter(|name| !monomorph.iter().any(|f| f.name == *name))
                .collect();
            if !missing.is_empty() {
                warn!(
                    "C++ bindings for {} (instantiated as {}) may be ill-formed: \
                     field(s) {} have zero-sized types.",
                    generic_name,
                    monomorph_name,
                    missing.join(", ")
                );
            }
        }

        for (g, m) in &self.structs {
            // Variant bodies are reported by the enum loop below, under the
            // enum's name.
            if g.is_enum_variant_body {
                continue;
            }
            warn_missing(g.path.name(), m.path.name(), &g.fields, &m.fields);
        }
        for (g, m) in &self.unions {
            warn_missing(g.path.name(), m.path.name(), &g.fields, &m.fields);
        }
        for (g, m) in &self.enums {
            for (gv, mv) in g.variants.iter().zip(&m.variants) {
                if let (VariantBody::Body { body: gb, .. }, VariantBody::Body { body: mb, .. }) =
                    (&gv.body, &mv.body)
                {
                    warn_missing(
                        &format!("{}::{}", g.path, gv.name),
                        &format!("{}::{}", m.path, mv.name),
                        &gb.fields,
                        &mb.fields,
                    );
                }
            }
        }
    }

    pub fn mangle_path(&self, path: &GenericPath) -> Option<&Path> {
        self.replacements.get(path)
    }

    pub fn drain_opaques(&mut self) -> Vec<OpaqueItem> {
        mem::take(&mut self.opaques)
    }

    pub fn drain_structs(&mut self) -> Vec<Struct> {
        mem::take(&mut self.structs)
            .into_iter()
            .map(|(_, monomorph)| monomorph)
            .collect()
    }

    pub fn drain_unions(&mut self) -> Vec<Union> {
        mem::take(&mut self.unions)
            .into_iter()
            .map(|(_, monomorph)| monomorph)
            .collect()
    }

    pub fn drain_typedefs(&mut self) -> Vec<Typedef> {
        mem::take(&mut self.typedefs)
    }

    pub fn drain_enums(&mut self) -> Vec<Enum> {
        mem::take(&mut self.enums)
            .into_iter()
            .map(|(_, monomorph)| monomorph)
            .collect()
    }
}

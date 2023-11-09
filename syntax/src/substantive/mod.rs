// Copyright (C) 2023 Tristan Gerritsen <tristan@thewoosh.org>
// All Rights Reserved.

use mango3_catalog::CatalogIndex;

/// Zelfstandig naamwoord, noun (substantive)
///
/// ## References
/// * [ANS 3. Het Substantief](https://e-ans.ivdnt.org/topics/pid/ans0301lingtopic)
#[derive(Debug, Clone)]
pub struct Substantive {
    pub catalog_index: CatalogIndex,
}

use crate::bindings as ll_bindings;
use crate::metadata;
use crate::IndividualId;
use crate::{tsk_flags_t, tsk_id_t, tsk_size_t, TskitError};

/// Row of a [`IndividualTable`]
pub struct IndividualTableRow {
    pub id: IndividualId,
    pub flags: tsk_flags_t,
    pub location: Option<Vec<f64>>,
    pub parents: Option<Vec<IndividualId>>,
    pub metadata: Option<Vec<u8>>,
}

impl PartialEq for IndividualTableRow {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
            && self.flags == other.flags
            && self.parents == other.parents
            && self.metadata == other.metadata
            && match &self.location {
                None => other.location.is_none(),
                Some(a) => match &other.location {
                    None => false,
                    Some(b) => {
                        if a.len() != b.len() {
                            false
                        } else {
                            for (i, j) in a.iter().enumerate() {
                                if !crate::util::f64_partial_cmp_equal(&j, &b[i]) {
                                    return false;
                                }
                            }
                            true
                        }
                    }
                },
            }
    }
}

/// An immutable view of a individual table.
///
/// These are not created directly.
/// Instead, use [`TableCollection::individuals`](crate::TableCollection::individuals)
/// to get a reference to an existing node table;
pub struct IndividualTable<'a> {
    table_: &'a ll_bindings::tsk_individual_table_t,
}

fn make_individual_table_row(table: &IndividualTable, pos: tsk_id_t) -> Option<IndividualTableRow> {
    if pos < table.num_rows() as tsk_id_t {
        let rv = IndividualTableRow {
            id: pos.into(),
            flags: table.flags(pos).unwrap(),
            location: table.location(pos).unwrap(),
            parents: table.parents(pos).unwrap(),
            metadata: table_row_decode_metadata!(table, pos),
        };
        Some(rv)
    } else {
        None
    }
}

pub type IndividualTableRefIterator<'a> =
    crate::table_iterator::TableIterator<&'a IndividualTable<'a>>;
pub type IndividualTableIterator<'a> = crate::table_iterator::TableIterator<IndividualTable<'a>>;

impl<'a> IndividualTable<'a> {
    pub(crate) fn new_from_table(individuals: &'a ll_bindings::tsk_individual_table_t) -> Self {
        IndividualTable {
            table_: individuals,
        }
    }

    /// Return the number of rows
    pub fn num_rows(&'a self) -> ll_bindings::tsk_size_t {
        self.table_.num_rows
    }

    /// Return the flags for a given row.
    ///
    /// # Errors
    ///
    /// * [`TskitError::IndexError`] if `row` is out of range.
    pub fn flags<I: Into<IndividualId> + Copy>(&self, row: I) -> Result<tsk_flags_t, TskitError> {
        unsafe_tsk_column_access!(row.into().0, 0, self.num_rows(), self.table_.flags)
    }

    /// Return the locations for a given row.
    ///
    /// # Errors
    ///
    /// * [`TskitError::IndexError`] if `row` is out of range.
    pub fn location<I: Into<IndividualId> + Copy>(
        &self,
        row: I,
    ) -> Result<Option<Vec<f64>>, TskitError> {
        unsafe_tsk_ragged_column_access!(
            row.into().0,
            0,
            self.num_rows(),
            self.table_.location,
            self.table_.location_offset,
            self.table_.location_length
        )
    }

    /// Return the parents for a given row.
    ///
    /// # Errors
    ///
    /// * [`TskitError::IndexError`] if `row` is out of range.
    pub fn parents<I: Into<IndividualId> + Copy>(
        &self,
        row: I,
    ) -> Result<Option<Vec<IndividualId>>, TskitError> {
        unsafe_tsk_ragged_column_access!(
            row.into().0,
            0,
            self.num_rows(),
            self.table_.parents,
            self.table_.parents_offset,
            self.table_.parents_length,
            IndividualId
        )
    }

    /// Return the metadata for a given row.
    ///
    /// # Errors
    ///
    /// * [`TskitError::IndexError`] if `row` is out of range.
    pub fn metadata<I: Into<IndividualId>, T: metadata::MetadataRoundtrip>(
        &'a self,
        row: I,
    ) -> Result<Option<T>, TskitError> {
        let buffer = metadata_to_vector!(self, row.into().0)?;
        decode_metadata_row!(T, buffer)
    }

    /// Return an iterator over rows of the table.
    /// The value of the iterator is [`IndividualTableRow`].
    ///
    pub fn iter(&self) -> IndividualTableRefIterator {
        crate::table_iterator::make_table_iterator::<&IndividualTable<'a>>(&self)
    }

    /// Return row `r` of the table.
    ///
    /// # Parameters
    ///
    /// * `r`: the row id.
    ///
    /// # Errors
    ///
    /// [`TskitError::IndexError`] if `r` is out of range.
    pub fn row<I: Into<IndividualId> + Copy>(
        &self,
        r: I,
    ) -> Result<IndividualTableRow, TskitError> {
        table_row_access!(r.into().0, self, make_individual_table_row)
    }
}

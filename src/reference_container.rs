#[derive(Default, Debug)]
pub struct ReferenceContainer<T> {
    data_index: Vec<usize>,
    id: Vec<usize>,
    data: Vec<T>,
    reference: Vec<usize>,
}

impl<T: Clone> Clone for ReferenceContainer<T> {
    fn clone(&self) -> Self {
        ReferenceContainer {
            data_index: self.data_index.clone(),
            id: self.id.clone(),
            data: self.data.clone(),
            reference: self.reference.clone(),
        }
    }
}

impl<T> ReferenceContainer<T> {
    pub fn new() -> Self {
        ReferenceContainer {
            data_index: Vec::new(),
            id: Vec::new(),
            data: Vec::new(),
            reference: Vec::new(),
        }
    }

    /// Finds the value associated with the given id and returns a reference
    /// to it. Returns `None` if the id is not found in the container.
    ///
    /// The method works by first searching for the index of the provided id
    /// in the 'id' vector, and then using that index to retrieve the
    /// corresponding value from the 'data' vector.
    pub fn get(&self, id: usize) -> Option<&T> {
        self.id
            .iter()
            .position(|&x| x == id)
            .and_then(|index| self.data.get(index))
    }

    /// Updates the value associated with the given id to the new data provided.
    /// Returns `Ok(())` if the update is successful, or an error message if
    /// the id is not found in the container or if the data index is out of bounds.
    pub fn update(&mut self, id: usize, new_data: T) -> Result<(), &'static str> {
        if let Some(index) = self.id.iter().position(|&x| x == id) {
            if let Some(data_ref) = self.data.get_mut(index) {
                *data_ref = new_data;
                Ok(())
            } else {
                Err("Data index out of bounds")
            }
        } else {
            Err("ID not found in the container")
        }
    }

    /// Retrieves the id associated with the given index. Returns
    /// `Ok(usize)' if the index is valid, or an error message if the index
    /// is out of bounds.
    pub fn get_id_from_index(&self, index: usize) -> Result<usize, &'static str> {
        self.id.get(index).copied().ok_or("Index out of bounds")
    }

    /// Retrieves the all ids on the same index as the given reference. Returns
    /// Some vector of references if the reference is valid, or None if the
    /// reference is not found in the container.
    pub fn get_ids_from_reference(&self, reference: usize) -> Option<Vec<usize>> {
        let mut ids = Vec::new();
        for (i, &ref_value) in self.reference.iter().enumerate() {
            if ref_value == reference
                && let Some(id) = self.id.get(i)
            {
                ids.push(*id);
            }
        }
        if ids.is_empty() { None } else { Some(ids) }
    }

    /// Finds the value associated with the given id and swaps it with the
    /// last element in the container, then removes the last element.
    /// Returns Ok(()) if the id is found and removed successfully, or an
    /// error message if the id is not found in the container.
    pub fn remove(&mut self, id: usize) -> Result<(), &'static str> {
        if let Some(index) = self.id.iter().position(|&x| x == id) {
            let last_index = self.data.len() - 1;

            self.swap(index, last_index)?;

            self.data.pop();
            self.reference.pop();

            Ok(())
        } else {
            Err("ID not found in the container")
        }
    }

    /// Adds a new element to the container and returns a reference to its
    /// associated id. If the container has space (i.e., the length of
    /// 'data' is less than the length of 'id'), it simply pushes the new
    /// data. Otherwise, it pushes the new data and also updates the 'id'
    /// and 'data_index' vectors accordingly. The method ensures that the
    /// new element is properly indexed and can be retrieved using its id in
    /// the future.
    pub fn add(&mut self, data: T, reference: usize) -> usize {
        let index = self.data.len();
        if self.data.len() < self.id.len() {
            self.data.push(data);
            self.reference.push(reference);
        } else {
            self.data.push(data);
            self.id.push(index);
            self.data_index.push(index);
            self.reference.push(reference);
        }
        self.id.get(index).copied().expect("This should never fail")
    }

    /// Swaps the elements at the specified indices in the container. This
    /// method keeps the integrity of the container by ensuring that the
    /// corresponding elements in the 'data', 'id', and
    /// 'reference' vectors are swapped together. It also updates the
    /// 'data_index' vector to reflect the new positions of the swapped elements.
    fn swap(&mut self, index_a: usize, index_b: usize) -> Result<(), &'static str> {
        self.data.swap(index_a, index_b);
        self.id.swap(index_a, index_b);
        self.reference.swap(index_a, index_b);

        let data_index_a = self.get_id_from_index(index_a)?;
        let data_index_b = self.get_id_from_index(index_b)?;

        self.data_index.swap(data_index_a, data_index_b);

        Ok(())
    }

    /// Returns the number of elements currently stored in the container by
    /// returning the length of the 'data' vector.
    pub fn size(&self) -> usize {
        self.data.len()
    }

    /// Checks if the container is empty by verifying if the 'data' vector has
    /// no elements. Returns `true` if the container is empty, and `false`
    /// otherwise.
    pub fn empty(&self) -> bool {
        self.data.is_empty()
    }

    /// Clears all elements from the container by clearing the 'data', 'id', and
    /// 'data_index' vectors. This effectively resets the container to an
    /// empty state, allowing it to be reused without any remaining data
    /// from previous operations.
    pub fn clear(&mut self) {
        self.data.clear();
        self.id.clear();
        self.data_index.clear();
        self.reference.clear();
    }
}

impl<T: Clone> ReferenceContainer<T> {
    /// Sorts the elements in the container based on their reference values. The
    /// method should rearrange the elements in the 'data', 'id', 'data_index',
    /// and 'reference' vectors to maintain the correct associations between
    /// ids, data, and references after sorting. The sorting can be done using
    /// any sorting algorithm, but it should ensure that the integrity of the
    /// container is maintained and that the elements are correctly ordered
    /// based on their reference values.
    pub fn sort(&mut self) {
        let mut combined: Vec<(usize, usize, T, usize)> = Vec::new();
        for i in 0..self.id.len() {
            if let (Some(id), Some(data), Some(reference)) =
                (self.id.get(i), self.data.get(i), self.reference.get(i))
            {
                combined.push((*id, i, data.clone(), *reference));
            }
        }

        combined.sort_by_key(|k| k.3);

        for (new_index, (id, _old_index, data, reference)) in combined.into_iter().enumerate() {
            self.id[new_index] = id;
            self.data[new_index] = data;
            self.reference[new_index] = reference;
            self.data_index[id] = new_index;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper function to create a sample container for testing purposes.
    fn setup_container() -> ReferenceContainer<String> {
        ReferenceContainer {
            data_index: vec![0, 1, 2],
            id: vec![0, 1, 2],
            data: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            reference: vec![0, 1, 1],
        }
    }

    fn setup_unsorted_container() -> ReferenceContainer<String> {
        ReferenceContainer {
            data_index: vec![0, 1, 2],
            id: vec![0, 1, 2],
            data: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            reference: vec![2, 0, 1],
        }
    }

    /// Tests the 'clone' method of the Container struct to ensure it creates a
    /// new instance of the container with the same data, id, data_index, and
    /// reference values as the original container. This test checks that the
    /// cloned container is identical to the original container in terms of its
    /// contents and structure, confirming that the cloning process is
    /// functioning correctly. It verifies that all the fields of the cloned
    /// container match those of the original container, ensuring that the clone
    /// is a true copy of the original. This test is crucial for validating the
    /// correctness of the Clone implementation for the Container struct.
    #[test]
    fn test_clone() {
        let container = setup_container();
        let cloned_container = container.clone();
        assert_eq!(container.data, cloned_container.data);
        assert_eq!(container.id, cloned_container.id);
        assert_eq!(container.data_index, cloned_container.data_index);
        assert_eq!(container.reference, cloned_container.reference);
    }

    /// Tests the 'get' method of the Container struct to ensure it
    /// correctly retrieves values based on their associated ids and returns
    /// `None` for ids that are not present in the container.
    #[test]
    fn test_get() {
        let container = setup_container();
        assert_eq!(container.get(0), Some(&"a".to_string()));
        assert_eq!(container.get(1), Some(&"b".to_string()));
        assert_eq!(container.get(2), Some(&"c".to_string()));
        assert_eq!(container.get(3), None);
    }

    /// Tests the 'update' method of the Container struct to ensure it
    /// correctly updates values based on their associated ids and returns
    /// appropriate error messages when the id is not found or when the data
    /// index is out of bounds.
    #[test]
    fn test_update() {
        let mut container = setup_container();
        assert_eq!(container.update(1, "updated".to_string()), Ok(()));
        assert_eq!(container.get(1), Some(&"updated".to_string()));
        assert_eq!(
            container.update(3, "new".to_string()),
            Err("ID not found in the container")
        );
        container.remove(2).unwrap();
        assert_eq!(
            container.update(2, "new".to_string()),
            Err("Data index out of bounds")
        );
    }

    /// Tests the reference related methods of the Container struct to ensure it
    /// correctly retrieves ids based on their associated reference and returns
    /// `None` for references that are not present in the container.
    #[test]
    fn test_reference_methods() {
        let container = setup_container();
        assert_eq!(container.get_id_from_index(1), Ok(1));
        assert_eq!(container.get_id_from_index(3), Err("Index out of bounds"));
        assert_eq!(container.get_ids_from_reference(1), Some(vec![1, 2]));
        assert_eq!(container.get_ids_from_reference(0), Some(vec![0]));
        assert_eq!(container.get_ids_from_reference(2), None);
    }

    /// Tests the 'remove' method of the Container struct to ensure it
    /// correctly removes elements based on their associated ids and handles
    /// cases where the id is not found in the container.
    #[test]
    fn test_remove() {
        let mut container = setup_container();
        assert_eq!(container.remove(2), Ok(()));
        assert_eq!(container.get(2), None);
        assert_eq!(container.remove(3), Err("ID not found in the container"));
    }

    /// Tests the 'add' method of the Container struct to ensure it
    /// correctly adds new elements to the container and returns the associated id
    /// for the newly added element. It also checks that the new element can be
    /// retrieved using its id after being added.
    #[test]
    fn test_add() {
        let mut container = setup_container();
        let new_id = container.add("d".to_string(), 5);
        assert_eq!(container.get(new_id), Some(&"d".to_string()));
        container.remove(1).unwrap();
        let new_id2 = container.add("e".to_string(), 6);
        assert_eq!(container.get(new_id2), Some(&"e".to_string()));
    }

    /// Tests the 'size' and 'empty' methods of the Container struct to ensure
    /// they correctly report the number of elements in the container and whether it is empty or not.
    #[test]
    fn test_size_and_empty() {
        let mut container = setup_container();
        assert_eq!(container.size(), 3);
        assert!(!container.empty());
        container.clear();
        assert_eq!(container.size(), 0);
        assert!(container.empty());
    }

    /// Test the sort method of the Container struct to ensure it correctly
    /// sorts the elements based on their reference values and maintains the
    /// integrity of the container. This test will check that after sorting, the
    /// elements are in the expected order based on their reference values and
    /// that the associations between ids, data, and references are preserved.
    #[test]
    fn test_sort() {
        let mut container = setup_unsorted_container();

        container.sort();

        assert_eq!(container.get(1), Some(&"b".to_string()));
        assert_eq!(container.get(2), Some(&"c".to_string()));
        assert_eq!(container.get(0), Some(&"a".to_string()));

        assert_eq!(container.get_ids_from_reference(0), Some(vec![1]));
        assert_eq!(container.get_ids_from_reference(1), Some(vec![2]));
        assert_eq!(container.get_ids_from_reference(2), Some(vec![0]));
    }
}

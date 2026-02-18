pub mod container {
    pub struct Container<T> {
        data_index: Vec<usize>,
        id: Vec<usize>,
        data: Vec<T>,
    }

    impl<T> Container<T> {
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
        /// `Ok(&usize)' if the index is valid, or an error message if the index
        /// is out of bounds.
        pub fn get_id_from_index(&self, index: usize) -> Result<&usize, &'static str> {
            self.id.get(index).ok_or("Index out of bounds")
        }

        /// Finds the value associated with the given id and swaps it with the
        /// last element in the container, then removes the last element.
        /// Returns Ok(()) if the id is found and removed successfully, or an
        /// error message if the id is not found in the container.
        pub fn remove(&mut self, id: usize) -> Result<(), &'static str> {
            if let Some(index) = self.id.iter().position(|&x| x == id) {
                let last_index = self.data.len() - 1;

                self.data.swap(index, last_index);
                self.id.swap(index, last_index);

                let data_index_a = *self.get_id_from_index(index)?;
                let data_index_b = *self.get_id_from_index(last_index)?;

                self.data_index.swap(data_index_a, data_index_b);

                self.data.pop();

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
        pub fn add(&mut self, data: T) -> &usize {
            let index = self.data.len();
            if self.data.len() < self.id.len() {
                self.data.push(data);
            } else {
                self.data.push(data);
                self.id.push(index);
                self.data_index.push(index);
            }
            self.id.get(index).expect("This should never fail")
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
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        /// Helper function to create a sample container for testing purposes.
        fn setup_container() -> Container<String> {
            Container {
                data_index: vec![0, 1, 2],
                id: vec![0, 1, 2],
                data: vec!["a".to_string(), "b".to_string(), "c".to_string()],
            }
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
            let new_id = *container.add("d".to_string());
            assert_eq!(container.get(new_id), Some(&"d".to_string()));
            container.remove(1).unwrap();
            let new_id2 = *container.add("e".to_string());
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
    }
}

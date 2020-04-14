pub fn is_sorted_by<F>(&self, mut compare: F) -> bool
   where
       F: FnMut(&T, &T) -> Option<Ordering>,
   {
       self.iter().is_sorted_by(|a, b| compare(*a, *b))
   }

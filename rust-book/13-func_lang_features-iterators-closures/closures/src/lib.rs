#[cfg(test)]
mod tests {
    // demonstrates limitation with cacher
    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }

    // to solve this, we would need a hash map
}

// These tests basically make sure that we
// can actually bind these things

#[cfg(test)]
mod tests
{
    use crate::*;

    #[test]
    fn test_node_is_sample()
    {
        let mut x = bindings::TSK_NODE_IS_SAMPLE;
        assert!(x > 0);
    }

    #[test]
    fn test_tsk_null()
    {
        assert_eq!(bindings::TSK_NULL, -1);
    }
}

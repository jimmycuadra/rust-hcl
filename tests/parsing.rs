extern crate hcl;

use std::fs::File;
use std::io::Read;

use hcl::parse;

macro_rules! test_fixture {
    ($id:ident, $name:expr, $is_ok:expr) => {
        #[test]
        fn $id() {
            let mut file = File::open(format!("tests/fixtures/{}.hcl", $name))
                .expect(&format!("failed to open tests/fixtures/{}.hcl", $name));
            let mut data = String::new();
            file.read_to_string(&mut data)
                .expect(&format!("failed to read tests/fixtures/{}.hcl", $name));
            let result = parse(&data);
            assert_eq!(
                result.is_ok(),
                $is_ok,
                "failed to parse tests/fixtures/{}.hcl: {:?}",
                $name,
                result
            );
        }
    }
}

test_fixture!(test_array_comment, "array_comment", true);
test_fixture!(test_assign_colon, "assign_colon", true);
test_fixture!(test_assign_deep, "assign_deep", true);
test_fixture!(test_basic, "basic", true);
test_fixture!(test_basic_int_string, "basic_int_string", true);
test_fixture!(test_basic_squish, "basic_squish", true);
test_fixture!(test_block_assign, "block_assign", false);
test_fixture!(test_comment, "comment", true);
test_fixture!(test_comment_single, "comment_single", true);
test_fixture!(test_complex, "complex", true);
test_fixture!(test_complex_key, "complex_key", true);
test_fixture!(test_decode_policy, "decode_policy", true);
test_fixture!(test_decode_tf_variable, "decode_tf_variable", true);
test_fixture!(test_empty, "empty", true);
test_fixture!(test_escape, "escape", true);
test_fixture!(test_escape_backslash, "escape_backslash", true);
test_fixture!(test_flat, "flat", true);
test_fixture!(test_float, "float", true);
test_fixture!(test_git_crypt, "git_crypt", false);
test_fixture!(test_list, "list", true);
test_fixture!(test_list_comma, "list_comma", true);
test_fixture!(test_list_of_lists, "list_of_lists", true);
test_fixture!(test_list_of_maps, "list_of_maps", true);
test_fixture!(test_multiline, "multiline", true);
test_fixture!(test_multiline_bad, "multiline_bad", false);
test_fixture!(test_multiline_indented, "multiline_indented", true);
test_fixture!(test_multiline_literal, "multiline_literal", false);
test_fixture!(test_multiline_literal_with_hil, "multiline_literal_with_hil", true);
test_fixture!(test_multiline_no_eof, "multiline_no_eof", true);
test_fixture!(test_multiline_no_hanging_indent, "multiline_no_hanging_indent", true);
test_fixture!(test_multiline_no_marker, "multiline_no_marker", false);
test_fixture!(test_multiple, "multiple", true);
test_fixture!(test_nested_block_comment, "nested_block_comment", true);
test_fixture!(test_nested_provider_bad, "nested_provider_bad", false);
test_fixture!(test_object_with_bool, "object_with_bool", true);
test_fixture!(test_old, "old", true);
test_fixture!(test_scientific, "scientific", true);
test_fixture!(test_slice_expand, "slice_expand", true);
test_fixture!(test_structure, "structure", true);
test_fixture!(test_structure2, "structure2", true);
test_fixture!(test_structure_basic, "structure_basic", true);
test_fixture!(test_structure_empty, "structure_empty", true);
test_fixture!(test_structure_flatmap, "structure_flatmap", true);
test_fixture!(test_structure_list, "structure_list", true);
test_fixture!(test_structure_multi, "structure_multi", true);
test_fixture!(test_terraform_heroku, "terraform_heroku", true);
test_fixture!(test_tfvars, "tfvars", true);
test_fixture!(test_types, "types", true);
test_fixture!(test_unterminated_block_comment, "unterminated_block_comment", false);
test_fixture!(test_unterminated_brace, "unterminated_brace", false);

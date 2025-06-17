use assert_cmd::Command;
use predicates::prelude::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TestCase<'a> {
    pub envs: Option<&'a HashMap<&'a str, &'a str>>,
    pub args: Vec<&'a str>,
    pub stdin: &'a str,
    pub stdout: &'a str,
    pub code: Option<i32>,
}

impl<'a> TestCase<'a> {
    pub fn run(self) {
        let mut cmd = Command::cargo_bin("bash-pinyin-completion-rs")
            .unwrap_or_else(|_| panic!("Failed to create command for test"));

        for (key, value) in self.envs.unwrap_or(&HashMap::new()) {
            cmd.env(key, value);
        }

        for arg in &self.args {
            cmd.arg(arg);
        }

        if !self.stdin.is_empty() {
            cmd.write_stdin(self.stdin);
        }

        let mut assertion = cmd.assert();

        if let Some(exit_code) = self.code {
            assertion = assertion.code(exit_code);
        } else {
            assertion = assertion.success();
        }

        assertion.stdout(predicate::eq(self.stdout));
    }
}

impl<'a> Default for TestCase<'a> {
    fn default() -> Self {
        TestCase {
            envs: None,
            args: Vec::new(),
            stdin: "",
            stdout: "",
            code: None,
        }
    }
}

#[test]
fn test_no_arguments() {
    TestCase {
        args: vec![],
        code: Some(1),
        ..Default::default()
    }
    .run();
}

#[test]
fn test_basic_pinyin_matching() {
    TestCase {
        args: vec!["ni"],
        stdin: "你好\n世界\n拼音\n测试\n",
        stdout: "你好\n",
        ..Default::default()
    }
    .run();
}

#[test]
fn test_pinyin_matching_with_multiple_candidates() {
    TestCase {
        args: vec!["shangh"],
        stdin: "上海\n深圳\n沈阳\n数据\n",
        stdout: "上海\n",
        ..Default::default()
    }
    .run();
}

#[test]
fn test_mixed() {
    TestCase {
        args: vec!["ce"],
        stdin: "测试\nhello\n世界\nworld\n测量\n",
        stdout: "测试\n测量\n",
        ..Default::default()
    }
    .run();
}

#[test]
fn test_prefix() {
    TestCase {
        args: vec!["py"],
        stdin: "拼音\n苹果\n朋友\n普通话\n",
        stdout: "拼音\n朋友\n",
        ..Default::default()
    }
    .run();

    TestCase {
        args: vec!["zhongg"],
        stdin: "中国\n知识\n质量\n重要\n",
        stdout: "中国\n",
        ..Default::default()
    }
    .run();
}

#[test]
fn test_environment_variable_quanpin_mode() {
    use std::collections::HashMap;
    let mut env_vars = HashMap::new();
    env_vars.insert("PINYIN_COMP_MODE", "Quanpin");

    TestCase {
        envs: Some(&env_vars),
        args: vec!["zhongguo"],
        stdin: "中国\n中心\n",
        stdout: "中国\n",
        ..Default::default()
    }
    .run();

    TestCase {
        envs: Some(&env_vars),
        args: vec!["zg"],
        stdin: "中国\n中心\n",
        stdout: "中国\n",
        ..Default::default()
    }
    .run();
}

#[test]
fn test_environment_variable_shuangpin_mode() {
    use std::collections::HashMap;
    let mut env_vars = HashMap::new();
    env_vars.insert("PINYIN_COMP_MODE", "ShuangpinXiaohe");

    TestCase {
        envs: Some(&env_vars),
        args: vec!["dl"],
        stdin: "中国\n大家\n",
        ..Default::default()
    }
    .run();

    TestCase {
        envs: Some(&env_vars),
        args: vec!["dajx"],
        stdin: "中国\n大家\n",
        stdout: "大家\n",
        ..Default::default()
    }
    .run();
}

#[test]
fn test_environment_variable_mix_mode() {
    use std::collections::HashMap;
    let mut env_vars = HashMap::new();
    env_vars.insert("PINYIN_COMP_MODE", "Quanpin,ShuangpinXiaohe");

    TestCase {
        envs: Some(&env_vars),
        args: vec!["zhongguo"],
        stdin: "中国\n中心\n",
        stdout: "中国\n",
        ..Default::default()
    }
    .run();

    TestCase {
        envs: Some(&env_vars),
        args: vec!["zg"],
        stdin: "中国\n中心\n",
        stdout: "",
        ..Default::default()
    }
    .run();

    TestCase {
        envs: Some(&env_vars),
        args: vec!["vsxb"],
        stdin: "中国\n中心\n",
        stdout: "中心\n",
        ..Default::default()
    }
    .run();
}

#[test]
fn test_environment_variable_multiple_shuangpin_mode() {
    use std::collections::HashMap;
    let mut env_vars = HashMap::new();
    env_vars.insert("PINYIN_COMP_MODE", "Quanpin,ShuangpinZrm,ShuangpinXiaohe");

    TestCase {
        envs: Some(&env_vars),
        args: vec!["udpn"],
        stdin: "双拼\n用户\n",
        stdout: "双拼\n",
        ..Default::default()
    }
    .run();

    TestCase {
        envs: Some(&env_vars),
        args: vec!["ulpb"],
        stdin: "双拼\n用户\n",
        stdout: "",
        ..Default::default()
    }
    .run();
}

#[test]
fn test_environment_variable_invalid_mode() {
    use std::collections::HashMap;
    let mut env_vars = HashMap::new();
    env_vars.insert("PINYIN_COMP_MODE", "Invalid");

    TestCase {
        envs: Some(&env_vars),
        args: vec!["shuangpin"],
        stdin: "双拼\n用户\n",
        stdout: "双拼\n",
        ..Default::default()
    }
    .run();

    TestCase {
        envs: Some(&env_vars),
        args: vec!["yh"],
        stdin: "双拼\n用户\n",
        stdout: "用户\n",
        ..Default::default()
    }
    .run();
}

#[test]
fn test_whitespace_handling() {
    TestCase {
        args: vec!["ni"],
        stdin: "你好 世界\n  中国  \n你好\n",
        stdout: "你好 世界\n你好\n",
        ..Default::default()
    }
    .run();
}

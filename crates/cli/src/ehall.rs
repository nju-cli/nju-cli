use std::collections::{HashMap, HashSet};

use anyhow::{Context, Result};
use clap::{Args, Subcommand};
use serde_json::Value;

use crate::auth;

#[derive(Debug, Subcommand)]
pub enum EhallCommand {
    /// 本科人才培养方案查询。
    #[command(name = "training-program")]
    TrainingProgram {
        #[command(subcommand)]
        command: TrainingProgramCommand,
    },
}

#[derive(Debug, Subcommand)]
pub enum TrainingProgramCommand {
    /// 列出所有专业培养方案。
    List(ListTrainingProgramsOptions),
    /// 查看某专业培养方案详情。
    Detail {
        /// 培养方案代码 PYFADM。
        program_id: String,
        /// 输出完整 JSON。
        #[arg(long)]
        json: bool,
    },
    /// 列出培养方案内的节点。
    Nodes {
        /// 培养方案代码 PYFADM。
        program_id: String,
        /// 输出完整 JSON。
        #[arg(long)]
        json: bool,
    },
    /// 查看培养方案内某个节点的详情。
    #[command(name = "node-detail")]
    NodeDetail {
        /// 培养方案代码 PYFADM。
        program_id: String,
        /// 节点代码 KZH。
        node_id: String,
        /// 输出完整 JSON。
        #[arg(long)]
        json: bool,
    },
    /// 查看培养方案内某个节点里的所有课程。
    Courses {
        /// 培养方案代码 PYFADM。
        program_id: String,
        /// 节点代码 KZH。
        node_id: String,
        /// 输出完整 JSON。
        #[arg(long)]
        json: bool,
    },
}

#[derive(Debug, Args)]
pub struct ListTrainingProgramsOptions {
    /// 每次请求页大小。
    #[arg(long, default_value_t = 200)]
    page_size: u64,
    /// 按培养方案名称模糊搜索。
    #[arg(long)]
    name: Option<String>,
    /// 年级代码，如 2026。
    #[arg(long)]
    grade: Option<String>,
    /// 院系代码 DWDM。
    #[arg(long)]
    department: Option<String>,
    /// 修读类型代码 XDLXDM，如 01。
    #[arg(long)]
    study_type: Option<String>,
    /// 输出完整 JSON。
    #[arg(long)]
    json: bool,
}

pub async fn handle(command: EhallCommand) -> Result<()> {
    let client = auth::authenticated_client()?;
    ehall::training_program::prepare_session(&client, ehall::training_program::default_role_id())
        .await
        .context("failed to prepare ehall session; try running `nju-cli login` again")?;

    match command {
        EhallCommand::TrainingProgram { command } => {
            handle_training_program(command, &client).await
        }
    }
}

async fn handle_training_program(
    command: TrainingProgramCommand,
    client: &reqwest::Client,
) -> Result<()> {
    match command {
        TrainingProgramCommand::List(options) => {
            let programs = ehall::training_program::list_all_training_programs(
                client,
                &ehall::training_program::TrainingProgramListOptions {
                    page_number: 1,
                    page_size: options.page_size,
                    name: options.name,
                    grade: options.grade,
                    department: options.department,
                    study_type: options.study_type,
                },
            )
            .await
            .context("failed to list training programs")?;

            if options.json {
                println!("{}", serde_json::to_string_pretty(&programs)?);
            } else {
                for program in programs {
                    println!(
                        "{}\t{}\t{}\t{}\t{}",
                        program.id,
                        program.name,
                        program.grade_display.unwrap_or_default(),
                        program.department_name.unwrap_or_default(),
                        program.major_name.unwrap_or_default()
                    );
                }
            }
        }
        TrainingProgramCommand::Detail { program_id, json } => {
            let program = ehall::training_program::get_training_program_detail(client, &program_id)
                .await
                .with_context(|| format!("failed to get training program {program_id}"))?;

            if json {
                println!("{}", serde_json::to_string_pretty(&program)?);
            } else {
                println!("{} {}", program.id, program.name);
                println!("年级: {}", program.grade_display.unwrap_or_default());
                println!("院系: {}", program.department_name.unwrap_or_default());
                println!("专业: {}", program.major_name.unwrap_or_default());
                println!("修读类型: {}", program.study_type_name.unwrap_or_default());
                if let Some(objective) = program.objective {
                    println!("\n培养目标:\n{objective}");
                }
                if let Some(requirement) = program.requirement {
                    println!("\n修读要求:\n{requirement}");
                }
                if let Some(graduation_requirement) = program.graduation_requirement {
                    println!("\n准出要求:\n{graduation_requirement}");
                }
            }
        }
        TrainingProgramCommand::Nodes { program_id, json } => {
            let nodes = ehall::training_program::list_training_program_nodes(client, &program_id)
                .await
                .with_context(|| {
                    format!("failed to list nodes of training program {program_id}")
                })?;

            if json {
                println!("{}", serde_json::to_string_pretty(&nodes)?);
            } else {
                print_training_program_nodes(&program_id, &nodes);
            }
        }
        TrainingProgramCommand::NodeDetail {
            program_id,
            node_id,
            json,
        } => {
            let node = ehall::training_program::get_training_program_node_detail(
                client,
                &program_id,
                &node_id,
            )
            .await
            .with_context(|| format!("failed to get detail of training program node {node_id}"))?;

            if json {
                println!("{}", serde_json::to_string_pretty(&node)?);
            } else {
                print_training_program_node_detail(&node);
            }
        }
        TrainingProgramCommand::Courses {
            program_id,
            node_id,
            json,
        } => {
            let courses =
                ehall::training_program::list_all_node_courses(client, &program_id, &node_id)
                    .await
                    .with_context(|| {
                        format!("failed to list courses of training program node {node_id}")
                    })?;

            if json {
                println!("{}", serde_json::to_string_pretty(&courses)?);
            } else {
                for course in courses {
                    println!(
                        "{}\t{}\t{}\t{}\t{}",
                        course.course_id,
                        course.course_name,
                        course.credits.unwrap_or_default(),
                        course.term.unwrap_or_default(),
                        course.department_name.unwrap_or_default()
                    );
                }
            }
        }
    }

    Ok(())
}

fn print_training_program_nodes(
    program_id: &str,
    nodes: &[ehall::training_program::TrainingProgramNode],
) {
    println!("PYFADM: {program_id}");
    println!("课程查询: nju-cli ehall training-program courses {program_id} <KZH>");
    println!("节点详情: nju-cli ehall training-program node-detail {program_id} <KZH>");
    println!();
    println!("节点树:");

    let id_set = nodes
        .iter()
        .map(|node| node.id.as_str())
        .collect::<HashSet<_>>();
    let mut children = HashMap::<String, Vec<usize>>::new();
    let mut roots = Vec::new();

    for (index, node) in nodes.iter().enumerate() {
        match normalized_parent_id(node.parent_id.as_deref()) {
            Some(parent_id) if id_set.contains(parent_id) => {
                children
                    .entry(parent_id.to_string())
                    .or_default()
                    .push(index);
            }
            _ => roots.push(index),
        }
    }

    let mut visited = HashSet::new();
    for root in roots {
        print_training_program_node(nodes, &children, root, 0, &mut visited);
    }

    for index in 0..nodes.len() {
        if !visited.contains(&index) {
            print_training_program_node(nodes, &children, index, 0, &mut visited);
        }
    }
}

fn print_training_program_node(
    nodes: &[ehall::training_program::TrainingProgramNode],
    children: &HashMap<String, Vec<usize>>,
    index: usize,
    depth: usize,
    visited: &mut HashSet<usize>,
) {
    if !visited.insert(index) {
        return;
    }

    let node = &nodes[index];
    let indent = "  ".repeat(depth);
    println!(
        "{indent}- {} {}{}",
        node.name,
        format_node_metadata(node),
        format_node_note_hint(node)
    );

    if let Some(child_indexes) = children.get(&node.id) {
        for &child_index in child_indexes {
            print_training_program_node(nodes, children, child_index, depth + 1, visited);
        }
    }
}

fn format_node_metadata(node: &ehall::training_program::TrainingProgramNode) -> String {
    let mut parts = vec![format!("KZH={}", node.id)];

    if let Some(node_type) = node
        .node_type_name
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        parts.push(format!("类型={node_type}"));
    }
    if let Some(category) = node
        .course_category_name
        .as_deref()
        .filter(|value| !value.is_empty())
    {
        parts.push(format!("类别={category}"));
    }
    if let Some(course_count) = node.course_count.as_ref().and_then(display_value) {
        parts.push(format!("课程数={course_count}"));
    }
    if let Some(required_credits) = node.required_credits.as_ref().and_then(display_value) {
        parts.push(format!("要求学分={required_credits}"));
    }
    if let Some(total_credits) = node.total_credits.as_ref().and_then(display_value) {
        parts.push(format!("合计学分={total_credits}"));
    }

    format!("({})", parts.join(", "))
}

fn format_node_note_hint(node: &ehall::training_program::TrainingProgramNode) -> &'static str {
    if has_node_notes(node) {
        " [node-detail有额外信息]"
    } else {
        ""
    }
}

fn has_node_notes(node: &ehall::training_program::TrainingProgramNode) -> bool {
    has_non_empty_text(node.requirement.as_deref())
        || has_non_empty_json_text(node.extra.get("XDYQ"))
        || has_non_empty_json_text(node.extra.get("BZ"))
}

fn print_training_program_node_detail(node: &ehall::training_program::TrainingProgramNode) {
    println!("PYFADM: {}", node.program_id);
    println!("KZH: {}", node.id);
    println!("节点名称: {}", node.name);

    print_optional_line("父节点 KZH", node.parent_id.as_deref());
    print_optional_line("节点类型", node.node_type_name.as_deref());
    print_optional_line("节点类型代码", node.node_type_id.as_deref());
    print_optional_line("课程类别", node.course_category_name.as_deref());
    print_optional_line("课程类别代码", node.course_category_id.as_deref());
    print_optional_value_line("课程数", node.course_count.as_ref());
    print_optional_value_line("要求学分", node.required_credits.as_ref());
    print_optional_value_line("合计学分", node.total_credits.as_ref());
    print_optional_value_line("最低修读学分", node.extra.get("ZDXDXF"));
    print_optional_value_line("总应修门数", node.extra.get("ZSXDMS"));
    print_optional_value_line("总完成课组数", node.extra.get("ZSWCKZS"));
    print_optional_extra_line("是否共享课组", node, "SFXGXKZ_DISPLAY");
    print_optional_extra_line("是否开课", node, "SFKK_DISPLAY");
    print_optional_extra_line("课程性质", node, "KCXZDM_DISPLAY");
    print_optional_extra_line("专业方向", node, "ZYFXDM_DISPLAY");
    print_optional_value_line("排序", node.extra.get("PX"));

    if let Some(requirement) = node
        .requirement
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        println!();
        println!("修读要求:");
        println!("{requirement}");
    }
    if let Some(requirement) = node
        .extra
        .get("XDYQ")
        .and_then(display_value)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
    {
        println!();
        println!("要求说明:");
        println!("{requirement}");
    }
    if let Some(remark) = node
        .extra
        .get("BZ")
        .and_then(display_value)
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
    {
        println!();
        println!("备注:");
        println!("{remark}");
    }
}

fn print_optional_line(label: &str, value: Option<&str>) {
    if let Some(value) = value.map(str::trim).filter(|value| !value.is_empty()) {
        println!("{label}: {value}");
    }
}

fn print_optional_value_line(label: &str, value: Option<&Value>) {
    if let Some(value) = value.and_then(display_value) {
        if !value.trim().is_empty() {
            println!("{label}: {value}");
        }
    }
}

fn print_optional_extra_line(
    label: &str,
    node: &ehall::training_program::TrainingProgramNode,
    key: &str,
) {
    print_optional_value_line(label, node.extra.get(key));
}

fn has_non_empty_text(value: Option<&str>) -> bool {
    value.map(str::trim).is_some_and(|value| !value.is_empty())
}

fn has_non_empty_json_text(value: Option<&Value>) -> bool {
    value
        .and_then(display_value)
        .is_some_and(|value| !value.trim().is_empty())
}

fn normalized_parent_id(parent_id: Option<&str>) -> Option<&str> {
    parent_id
        .map(str::trim)
        .filter(|parent_id| !parent_id.is_empty() && *parent_id != "-1")
}

fn display_value(value: &Value) -> Option<String> {
    match value {
        Value::Null => None,
        Value::String(value) if value.is_empty() => None,
        Value::String(value) => Some(value.clone()),
        Value::Number(value) => Some(value.to_string()),
        Value::Bool(value) => Some(value.to_string()),
        _ => Some(value.to_string()),
    }
}

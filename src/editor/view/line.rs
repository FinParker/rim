/*
 * @Author: iming 2576226012@qq.com
 * @Date: 2025-06-22 15:36:55
 * @LastEditors: iming 2576226012@qq.com
 * @LastEditTime: 2025-06-23 14:07:14
 * @FilePath: \rim\src\editor\view\line.rs
 * @Description: 行处理, 支持字素切分
 */
use std::{cmp, ops::Range};

use crossterm::{cursor, terminal::WindowSize};
use unicode_segmentation::UnicodeSegmentation;
use unicode_width::UnicodeWidthStr;

#[derive(Debug, Clone, Copy, PartialEq)]
enum GraphemeWidth {
    Half, // 半角字符
    Full, // 全角字符
}

#[derive(Debug)]
struct TextFragment {
    grapheme: String,
    rendered_width: GraphemeWidth,
    replacement: Option<char>,
}

impl TextFragment {
    /// 获取实际渲染的字符
    fn display_char(&self) -> char {
        self.replacement.unwrap_or_else(|| {
            // 如果没有替换字符，使用原始字素的第一个字符
            // 注意：字素可能是多字符组合（如emoji），但渲染时我们只取第一个字符
            self.grapheme.chars().next().unwrap_or(' ')
        })
    }

    /// 获取实际渲染的宽度
    fn display_width(&self) -> usize {
        match self.rendered_width {
            GraphemeWidth::Half => 1,
            GraphemeWidth::Full => 2,
        }
    }
}

pub struct Line {
    string: String,
    fragments: Vec<TextFragment>,
}

impl Line {
    pub fn from(line_str: &str) -> Self {
        if line_str.is_empty() {
            return Self {
                string: String::new(),
                fragments: Vec::new(),
            };
        }
        let fragments = UnicodeSegmentation::graphemes(line_str, true)
            .map(|grapheme| {
                let width = grapheme.width();
                let rendered_width: GraphemeWidth;
                let replacement: Option<char>;
                if width == 0 {
                    rendered_width = GraphemeWidth::Half;
                    replacement = Some('·');
                } else {
                    if width == 1 {
                        rendered_width = GraphemeWidth::Half;
                        replacement = None;
                    } else {
                        rendered_width = GraphemeWidth::Full;
                        replacement = None;
                    }
                }
                TextFragment {
                    grapheme: grapheme.to_string(),
                    rendered_width,
                    replacement,
                }
            })
            .collect();
        Self {
            string: String::from(line_str),
            fragments,
        }
    }

    pub fn as_str(&self) -> &str {
        &self.string
    }

    pub fn to_string(&self) -> String {
        self.string.clone()
    }

    pub fn fragments(&self) -> &[TextFragment] {
        &self.fragments
    }
    pub fn get_display_string(&self, range: Range<usize>) -> String {
        if self.is_empty() {
            return String::new();
        }

        let start = range.start;
        let end = range.end;

        // 空范围检查
        if start >= end {
            return String::new();
        }

        let mut result = String::new();
        let mut cur_pos = 0; // 当前显示位置

        for frag in &self.fragments {
            // 计算片段起始位置
            let frag_start = cur_pos;

            // 计算片段结束位置
            let frag_end = match frag.rendered_width {
                GraphemeWidth::Half => frag_start + 1,
                GraphemeWidth::Full => frag_start + 2,
            };

            // 片段在显示范围之后 - 停止处理
            if frag_start >= end {
                break;
            }

            // 片段与显示范围相交
            if frag_end > start {
                // 检查是否跨越左边界（start）
                if frag_start < start
                    && frag_end > start
                    && frag.rendered_width == GraphemeWidth::Full
                {
                    result.push('·'); // 左边界截断指示
                }
                // 检查是否在显示范围内
                else if frag_start >= start && frag_end <= end {
                    result.push(frag.display_char());
                }
                // 检查是否跨越右边界（end）
                else if frag_start < end
                    && frag_end > end
                    && frag.rendered_width == GraphemeWidth::Full
                {
                    result.push('·'); // 右边界截断指示
                }
            }

            // 更新当前位置
            cur_pos = frag_end;

            // 如果当前位置已超过显示范围结束位置，停止处理
            if cur_pos >= end {
                break;
            }
        }

        result
    }

    pub fn byte_len(&self) -> usize {
        let mut byte_len = 0;
        for frag in &self.fragments {
            byte_len += match frag.rendered_width {
                GraphemeWidth::Half => 1,
                GraphemeWidth::Full => 2,
            }
        }
        byte_len
    }

    pub fn fragment_len(&self) -> usize {
        self.fragments.len()
    }

    pub fn is_empty(&self) -> bool {
        self.string.is_empty()
    }
}

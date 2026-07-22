//! Mirror of `PortholeCore/ProcessNameHeuristic.swift`.

/// Ordered (needle, label). First needle found in the full command line wins.
const RULES: &[(&str, &str)] = &[
    ("vite", "vite"),
    ("next-server", "next"),
    ("next dev", "next"),
    ("nuxt", "nuxt"),
    ("astro", "astro"),
    ("webpack", "webpack"),
    ("remix", "remix"),
    ("uvicorn", "uvicorn"),
    ("gunicorn", "gunicorn"),
    ("flask", "flask"),
    ("manage.py runserver", "django"),
    ("rails", "rails"),
    ("prisma", "prisma"),
    ("storybook", "storybook"),
];

pub fn display_name(command: &str, full_command: &str) -> String {
    let haystack = full_command.to_lowercase();
    for (needle, label) in RULES {
        if haystack.contains(needle) {
            return (*label).to_string();
        }
    }
    command.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_known_dev_tools() {
        assert_eq!(
            display_name("node", "node /home/x/app/node_modules/.bin/vite"),
            "vite"
        );
        assert_eq!(display_name("node", "next-server (v14.2.0)"), "next");
        assert_eq!(
            display_name("python3.12", "/usr/bin/python3.12 -m uvicorn main:app"),
            "uvicorn"
        );
    }

    #[test]
    fn falls_back_to_command() {
        assert_eq!(display_name("redis-server", "redis-server *:6379"), "redis-server");
    }
}

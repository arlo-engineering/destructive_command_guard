//! Supabase CLI patterns - protections against destructive Supabase commands.
//!
//! This includes patterns for:
//! - `supabase db reset` (drops and recreates the database)
//! - `supabase db push` (pushes migrations to remote)
//! - `supabase migration repair` (modifies migration history)
//! - `supabase db shell` with destructive SQL (DROP/TRUNCATE/DELETE)
//! - `supabase projects delete` / `supabase orgs delete`

use crate::packs::{DestructivePattern, Pack, PatternSuggestion, SafePattern};
use crate::{destructive_pattern, safe_pattern};

// ============================================================================
// Suggestion constants (must be 'static for the pattern struct)
// ============================================================================

/// Suggestions for `supabase db reset` pattern.
const DB_RESET_SUGGESTIONS: &[PatternSuggestion] = &[
    PatternSuggestion::new(
        "supabase db dump -f backup.sql",
        "Dump the database before resetting",
    ),
    PatternSuggestion::new(
        "supabase db diff",
        "Review schema differences before resetting",
    ),
    PatternSuggestion::new(
        "supabase migration list",
        "Check migration status before resetting",
    ),
];

/// Suggestions for `supabase db push` pattern.
const DB_PUSH_SUGGESTIONS: &[PatternSuggestion] = &[
    PatternSuggestion::new(
        "supabase db push --dry-run",
        "Preview migration changes without applying them",
    ),
    PatternSuggestion::new(
        "supabase db diff",
        "Review schema differences before pushing",
    ),
    PatternSuggestion::new(
        "supabase db dump -f backup.sql --linked",
        "Dump the remote database before pushing migrations",
    ),
];

/// Suggestions for `supabase migration repair` pattern.
const MIGRATION_REPAIR_SUGGESTIONS: &[PatternSuggestion] = &[
    PatternSuggestion::new(
        "supabase migration list",
        "Review current migration status before repairing",
    ),
    PatternSuggestion::new(
        "supabase db dump -f backup.sql",
        "Dump the database before modifying migration history",
    ),
];

/// Suggestions for `supabase db shell` with destructive SQL.
const DB_SHELL_DESTRUCTIVE_SUGGESTIONS: &[PatternSuggestion] = &[
    PatternSuggestion::new(
        "supabase db dump -f backup.sql",
        "Dump the database before running destructive SQL",
    ),
    PatternSuggestion::new(
        "supabase db shell -- -c 'SELECT COUNT(*) FROM {tablename}'",
        "Check row count before deleting",
    ),
];

/// Suggestions for `supabase projects delete` pattern.
const PROJECTS_DELETE_SUGGESTIONS: &[PatternSuggestion] = &[
    PatternSuggestion::new(
        "supabase db dump -f backup.sql --linked",
        "Dump the database before deleting the project",
    ),
    PatternSuggestion::new(
        "supabase projects list",
        "List projects to verify the correct one",
    ),
];

/// Suggestions for `supabase orgs delete` pattern.
const ORGS_DELETE_SUGGESTIONS: &[PatternSuggestion] = &[
    PatternSuggestion::new(
        "supabase orgs list",
        "List organizations to verify the correct one",
    ),
    PatternSuggestion::new(
        "supabase projects list",
        "List projects in the organization before deleting",
    ),
];

/// Create the Supabase pack.
#[must_use]
pub fn create_pack() -> Pack {
    Pack {
        id: "database.supabase".to_string(),
        name: "Supabase",
        description: "Protects against destructive Supabase CLI operations like db reset, \
                      db push, migration repair, and project deletion",
        keywords: &[
            "supabase",
            "db reset",
            "db push",
            "migration repair",
            "projects delete",
            "orgs delete",
        ],
        safe_patterns: create_safe_patterns(),
        destructive_patterns: create_destructive_patterns(),
        keyword_matcher: None,
        safe_regex_set: None,
        safe_regex_set_is_complete: false,
    }
}

fn create_safe_patterns() -> Vec<SafePattern> {
    vec![
        // supabase db diff is safe (shows schema differences)
        safe_pattern!("supabase-db-diff", r"supabase\s+db\s+diff"),
        // supabase db lint is safe (checks for issues)
        safe_pattern!("supabase-db-lint", r"supabase\s+db\s+lint"),
        // supabase status is safe (shows project status)
        safe_pattern!("supabase-status", r"supabase\s+status"),
        // supabase db shell without destructive SQL is safe
        safe_pattern!(
            "supabase-db-shell-safe",
            r"(?i)supabase\s+db\s+shell\s*$"
        ),
        // supabase migration list is safe
        safe_pattern!(
            "supabase-migration-list",
            r"supabase\s+migration\s+list"
        ),
        // supabase migration new is safe (creates a new migration file)
        safe_pattern!(
            "supabase-migration-new",
            r"supabase\s+migration\s+new"
        ),
    ]
}

fn create_destructive_patterns() -> Vec<DestructivePattern> {
    vec![
        // supabase db reset — drops and recreates the database
        destructive_pattern!(
            "supabase-db-reset",
            r"supabase\s+db\s+reset",
            "supabase db reset drops and recreates the entire database. All data will be lost.",
            Critical,
            "supabase db reset completely destroys and recreates your database:\n\n\
             - All tables, views, and functions are dropped\n\
             - All data is permanently deleted\n\
             - Migrations are re-applied from scratch\n\
             - Seed data (if configured) is re-inserted\n\n\
             This is irreversible for any data not captured in migrations or seeds.\n\n\
             Before resetting:\n  \
             supabase db dump -f backup.sql\n\n\
             Review differences first:\n  \
             supabase db diff",
            DB_RESET_SUGGESTIONS
        ),
        // supabase db push — pushes migrations to remote
        destructive_pattern!(
            "supabase-db-push",
            r"supabase\s+db\s+push",
            "supabase db push applies migrations to the remote database. Use --dry-run to preview first.",
            Critical,
            "supabase db push applies pending migrations to the remote (linked) database:\n\n\
             - Migrations may include DROP/ALTER statements\n\
             - Changes to the production database are irreversible\n\
             - With --linked, this targets the live project database\n\n\
             Always preview changes first:\n  \
             supabase db push --dry-run\n\n\
             Dump remote database before pushing:\n  \
             supabase db dump -f backup.sql --linked",
            DB_PUSH_SUGGESTIONS
        ),
        // supabase migration repair — modifies migration history
        destructive_pattern!(
            "supabase-migration-repair",
            r"supabase\s+migration\s+repair",
            "supabase migration repair modifies the migration history. This can cause drift between schema and migrations.",
            Critical,
            "supabase migration repair alters the migration history table:\n\n\
             - Can mark migrations as applied or reverted\n\
             - May cause schema drift if used incorrectly\n\
             - Can break future migration runs\n\n\
             Review migration status first:\n  \
             supabase migration list\n\n\
             Dump the database before repairing:\n  \
             supabase db dump -f backup.sql",
            MIGRATION_REPAIR_SUGGESTIONS
        ),
        // supabase db shell with destructive SQL (DROP/TRUNCATE/DELETE)
        destructive_pattern!(
            "supabase-db-shell-destructive",
            r"(?i)supabase\s+db\s+shell\s+.*\b(DROP|TRUNCATE|DELETE)\b",
            "supabase db shell with destructive SQL (DROP/TRUNCATE/DELETE). Verify the command carefully.",
            High,
            "supabase db shell is being invoked with destructive SQL:\n\n\
             - DROP permanently removes database objects\n\
             - TRUNCATE removes all rows from tables\n\
             - DELETE without WHERE removes all rows\n\n\
             Dump the database before running destructive commands:\n  \
             supabase db dump -f backup.sql\n\n\
             Check row count first:\n  \
             supabase db shell -- -c 'SELECT COUNT(*) FROM tablename'",
            DB_SHELL_DESTRUCTIVE_SUGGESTIONS
        ),
        // supabase projects delete
        destructive_pattern!(
            "supabase-projects-delete",
            r"supabase\s+projects\s+delete",
            "supabase projects delete permanently removes the entire Supabase project and all its data.",
            High,
            "supabase projects delete permanently removes a Supabase project:\n\n\
             - The database and all data are deleted\n\
             - Auth users and sessions are removed\n\
             - Storage buckets and files are deleted\n\
             - Edge functions are removed\n\
             - API keys are invalidated\n\n\
             This action cannot be undone.\n\n\
             Dump the database before deleting:\n  \
             supabase db dump -f backup.sql --linked\n\n\
             Verify the project:\n  \
             supabase projects list",
            PROJECTS_DELETE_SUGGESTIONS
        ),
        // supabase orgs delete
        destructive_pattern!(
            "supabase-orgs-delete",
            r"supabase\s+orgs\s+delete",
            "supabase orgs delete permanently removes the organization and may affect all projects within it.",
            High,
            "supabase orgs delete permanently removes a Supabase organization:\n\n\
             - All projects in the organization may be affected\n\
             - Billing and subscription are cancelled\n\
             - Team members lose access\n\n\
             This action cannot be undone.\n\n\
             List organization projects first:\n  \
             supabase projects list\n\n\
             Verify the organization:\n  \
             supabase orgs list",
            ORGS_DELETE_SUGGESTIONS
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::packs::test_helpers::*;

    #[test]
    fn test_db_reset() {
        let pack = create_pack();
        assert_blocks(&pack, "supabase db reset", "db reset");
        assert_blocks(&pack, "supabase  db  reset", "db reset");
    }

    #[test]
    fn test_db_push() {
        let pack = create_pack();
        assert_blocks(&pack, "supabase db push", "db push");
        assert_blocks(&pack, "supabase db push --linked", "db push");
        assert_blocks(&pack, "supabase db push --dry-run", "db push");
    }

    #[test]
    fn test_migration_repair() {
        let pack = create_pack();
        assert_blocks(&pack, "supabase migration repair", "migration repair");
        assert_blocks(
            &pack,
            "supabase migration repair --status applied",
            "migration repair",
        );
    }

    #[test]
    fn test_db_shell_destructive() {
        let pack = create_pack();
        assert_blocks(
            &pack,
            "supabase db shell -- -c 'DROP TABLE users'",
            "destructive SQL",
        );
        assert_blocks(
            &pack,
            "supabase db shell -- -c 'TRUNCATE users'",
            "destructive SQL",
        );
        assert_blocks(
            &pack,
            "supabase db shell -- -c 'DELETE FROM users'",
            "destructive SQL",
        );
    }

    #[test]
    fn test_projects_delete() {
        let pack = create_pack();
        assert_blocks(&pack, "supabase projects delete", "projects delete");
        assert_blocks(
            &pack,
            "supabase projects delete --ref abc123",
            "projects delete",
        );
    }

    #[test]
    fn test_orgs_delete() {
        let pack = create_pack();
        assert_blocks(&pack, "supabase orgs delete", "orgs delete");
        assert_blocks(&pack, "supabase orgs delete --id org123", "orgs delete");
    }

    #[test]
    fn test_safe_commands() {
        let pack = create_pack();
        assert_allows(&pack, "supabase db diff");
        assert_allows(&pack, "supabase db lint");
        assert_allows(&pack, "supabase status");
        assert_allows(&pack, "supabase db shell");
        assert_allows(&pack, "supabase migration list");
        assert_allows(&pack, "supabase migration new create_users");
    }
}

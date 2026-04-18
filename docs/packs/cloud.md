# Cloud Provider Packs

This document describes packs in the `cloud` category.

## Packs in this Category

- [AWS CLI](#cloudaws)
- [Google Cloud SDK](#cloudgcp)
- [Azure CLI](#cloudazure)

---

## AWS CLI

**Pack ID:** `cloud.aws`

Protects against destructive AWS CLI operations like terminate-instances, delete-db-instance, s3 rm --recursive, Athena/Glue catalog deletions, and destructive Athena queries (DROP, TRUNCATE, DELETE without WHERE)

### Keywords

Commands containing these keywords are checked against this pack:

- `aws`
- `terminate`
- `delete`
- `s3`
- `ec2`
- `rds`
- `ecr`
- `logs`
- `athena`
- `glue`
- `kms`
- `secretsmanager`
- `route53`
- `cloudtrail`
- `redshift`
- `kinesis`
- `efs`

### Safe Patterns (Allowed)

These patterns match safe commands that are always allowed:

| Pattern Name | Pattern |
|--------------|----------|
| `aws-describe` | `aws\s+\S+\s+describe-` |
| `aws-list` | `aws\s+\S+\s+list-` |
| `aws-get` | `aws\s+\S+\s+get-` |
| `s3-ls` | `aws\s+s3\s+ls` |
| `s3-cp` | `aws\s+s3\s+cp` |
| `aws-dry-run` | `aws\s+.*--dry-run` |
| `sts-identity` | `aws\s+sts\s+get-caller-identity` |
| `cfn-describe` | `aws\s+cloudformation\s+(?:describe\|list)-` |
| `ecr-login` | `aws\s+ecr\s+get-login` |
| `athena-delete-with-where` | Athena `DELETE FROM <table> ... WHERE` (targeted deletion, no trailing `;`) — the only Athena safe pattern needed, since it escapes the broad `athena-query-delete-without-where` destructive rule. Pure `SELECT` / `SHOW` / `DESCRIBE` / `EXPLAIN` / `CREATE` / `INSERT` / `UPDATE` queries aren't matched by any destructive rule and are allowed by default. |

### Destructive Patterns (Blocked)

These patterns match potentially destructive commands:

| Pattern Name | Reason | Severity |
|--------------|--------|----------|
| `ec2-terminate` | aws ec2 terminate-instances permanently destroys EC2 instances. | critical |
| `removes AWS resources` | aws ec2 delete-* permanently removes AWS resources. | high |
| `s3-rm-recursive` | aws s3 rm --recursive permanently deletes all objects in the path. | critical |
| `s3-rb` | aws s3 rb removes the entire S3 bucket. | critical |
| `s3api-delete-bucket` | aws s3api delete-bucket removes the entire S3 bucket. | critical |
| `rds-delete` | aws rds delete-* permanently destroys the database resource (instance, cluster, snapshot, parameter group, subnet group, etc.). | critical |
| `cfn-delete-stack` | aws cloudformation delete-stack removes the entire stack and its resources. | critical |
| `lambda-delete` | aws lambda delete-* permanently removes a Lambda resource (function, alias, layer version, event source mapping, etc.). | high |
| `iam-delete` | aws iam delete-* removes IAM resources. Verify dependencies first. | high |
| `dynamodb-delete` | aws dynamodb delete-table permanently deletes the table and all data. | critical |
| `eks-delete` | aws eks delete-cluster removes the entire EKS cluster. | critical |
| `ecr-delete-repository` | aws ecr delete-repository permanently deletes the repository and its images. | high |
| `ecr-batch-delete-image` | aws ecr batch-delete-image permanently deletes one or more images. | high |
| `ecr-delete-lifecycle-policy` | aws ecr delete-lifecycle-policy removes the repository lifecycle policy. | medium |
| `logs-delete-log-group` | aws logs delete-log-group permanently deletes a log group and all events. | high |
| `logs-delete-log-stream` | aws logs delete-log-stream permanently deletes a log stream and all events. | high |
| `kms-schedule-key-deletion` | aws kms schedule-key-deletion schedules a KMS key for irreversible deletion — all data encrypted with it becomes unreadable. | critical |
| `secretsmanager-delete-secret` | aws secretsmanager delete-secret destroys a stored secret — typically irrecoverable credentials. | critical |
| `route53-delete-hosted-zone` | aws route53 delete-hosted-zone removes a DNS zone — domains stop resolving. | critical |
| `cloudtrail-delete-trail` | aws cloudtrail delete-trail removes an audit trail — compliance/forensics impact. | critical |
| `redshift-delete-cluster` | aws redshift delete-cluster destroys a Redshift cluster and all loaded data. | critical |
| `kinesis-delete-stream` | aws kinesis delete-stream destroys a data stream — in-flight records are lost. | critical |
| `efs-delete-file-system` | aws efs delete-file-system destroys an EFS filesystem — all files and mount targets are lost. | critical |
| `s3api-delete-object` | aws s3api delete-object deletes an S3 object — object is gone unless bucket versioning is enabled. | high |
| `athena-delete-data-catalog` | aws athena delete-data-catalog removes the catalog and all database/table definitions tied to it. | critical |
| `athena-delete-work-group` | aws athena delete-work-group removes the Athena workgroup and its configuration. | high |
| `athena-delete-named-query` | aws athena delete-named-query permanently removes a saved query. | medium |
| `athena-query-drop-database` | Athena `DROP DATABASE`/`SCHEMA` removes the database from the Glue catalog. | critical |
| `athena-query-drop-table` | Athena `DROP TABLE`/`VIEW` removes the table definition from the Glue catalog. | high |
| `athena-query-truncate` | Athena `TRUNCATE TABLE` deletes all rows from an Iceberg table. | critical |
| `athena-query-string-from-file` | Athena `--query-string file://…`/`fileb://…` loads the SQL from disk, so DCG can't grep the statement. Use inline `--query-string '…'` instead. | high |
| `athena-cli-input-file` | Athena `start-query-execution --cli-input-json file://…`/`--cli-input-yaml file://…` loads the full invocation from disk, hiding `QueryString` from inspection. Inline `--cli-input-json '{…}'` is still allowed because the broad `DROP`/`TRUNCATE`/`DELETE` rules can still see its contents. | high |
| `athena-query-delete-without-where` | Athena `DELETE` without a `WHERE` clause removes all rows from the target table. | critical |
| `glue-delete-database` | aws glue delete-database removes the database and every table definition inside it. | critical |
| `glue-delete-table` | aws glue delete-table removes the table definition from the catalog. | high |
| `glue-batch-delete-table` | aws glue batch-delete-table removes multiple table definitions in one call. | critical |
| `glue-delete-partition` | aws glue delete-partition removes partition metadata. | high |
| `glue-batch-delete-partition` | aws glue batch-delete-partition removes multiple partition definitions in one call. | high |
| `glue-delete-crawler` | aws glue delete-crawler removes the crawler configuration. | medium |
| `glue-delete-job` | aws glue delete-job removes the ETL job definition and all run history. | high |
| `glue-delete-dev-endpoint` | aws glue delete-dev-endpoint tears down the development endpoint. | medium |

### Allowlist Guidance

To allowlist a specific rule from this pack, add to your allowlist:

```toml
[[allow]]
rule = "cloud.aws:<pattern-name>"
reason = "Your reason here"
```

To allowlist all rules from this pack (use with caution):

```toml
[[allow]]
rule = "cloud.aws:*"
reason = "Your reason here"
risk_acknowledged = true
```

---

## Google Cloud SDK

**Pack ID:** `cloud.gcp`

Protects against destructive gcloud operations like instances delete, sql instances delete, and gsutil rm -r

### Keywords

Commands containing these keywords are checked against this pack:

- `gcloud`
- `gsutil`
- `delete`
- `instances`
- `artifacts`
- `images`
- `repositories`
- `bq`
- `secrets`
- `kms`
- `iam`
- `dns`
- `spanner`
- `bigtable`
- `dataproc`

### Safe Patterns (Allowed)

These patterns match safe commands that are always allowed. Every rule allows global flags (`--project`, `--account`, `--impersonate-service-account`, …) to appear between `gcloud`/`gsutil` and the subcommand. Pattern-wise the subcommand literal is preceded by whitespace so flag values like `--config-file` or `--auth-token` cannot false-match a safe rule on a destructive command.

| Pattern Name | Pattern |
|--------------|----------|
| `gcloud-describe` | `gcloud\b.*?\s+\S+\s+\S+\s+describe` |
| `gcloud-list` | `gcloud\b.*?\s+\S+\s+\S+\s+list` |
| `gsutil-ls` | `gsutil\b.*?\bls` |
| `gsutil-cp` | `gsutil\b.*?\bcp` |
| `gcloud-config` | `gcloud\b.*?\s+config\b` |
| `gcloud-auth` | `gcloud\b.*?\s+auth\b` |
| `gcloud-info` | `gcloud\b.*?\s+info\b` |

### Destructive Patterns (Blocked)

These patterns match potentially destructive commands. Every rule matches through wrapper tools (`gcloud-vault`-like binaries) and with arbitrary global flags (`--project X`, `--quiet`, `--verbosity debug`, …) between `gcloud` and the service name.

| Pattern Name | Reason | Severity |
|--------------|--------|----------|
| `compute-delete` | gcloud compute instances delete permanently destroys VM instances. | critical |
| `disk-delete` | gcloud compute disks delete permanently destroys disk data. | critical |
| `sql-delete` | gcloud sql instances delete permanently destroys the Cloud SQL instance. | critical |
| `gsutil-rm-recursive` | gsutil rm -r permanently deletes all objects in the path. | critical |
| `gsutil-rb` | gsutil rb removes the entire GCS bucket. | critical |
| `gke-delete` | gcloud container clusters delete removes the entire GKE cluster. | critical |
| `project-delete` | gcloud projects delete removes the entire GCP project and ALL its resources! | critical |
| `functions-delete` | gcloud functions delete removes the Cloud Function. | high |
| `pubsub-delete` | gcloud pubsub delete removes Pub/Sub topics or subscriptions. | high |
| `firestore-delete` | gcloud firestore delete removes Firestore data. | critical |
| `container-images-delete` | gcloud container images delete permanently deletes container images. | high |
| `artifacts-docker-images-delete` | gcloud artifacts docker images delete permanently deletes container images. | high |
| `artifacts-repositories-delete` | gcloud artifacts repositories delete permanently deletes the repository. | critical |
| `secrets-delete` | gcloud secrets delete destroys a Secret Manager secret — credentials gone. | critical |
| `kms-keys-destroy` | gcloud kms keys versions destroy schedules a CryptoKeyVersion for destruction — data encrypted with it becomes unrecoverable. | critical |
| `iam-service-accounts-delete` | gcloud iam service-accounts delete removes a service account — workloads authenticating with it break. | critical |
| `iam-roles-delete` | gcloud iam roles delete removes a custom IAM role — users/SAs bound to it lose permissions. | high |
| `dns-managed-zones-delete` | gcloud dns managed-zones delete removes a DNS zone — domains stop resolving. | critical |
| `logging-sinks-delete` | gcloud logging sinks delete removes an audit-log export — compliance/forensics impact. | high |
| `spanner-instances-delete` | gcloud spanner instances delete destroys a Spanner instance — all databases and data lost. | critical |
| `bigtable-instances-delete` | gcloud bigtable instances delete destroys a Bigtable instance — all tables and data lost. | critical |
| `dataproc-clusters-delete` | gcloud dataproc clusters delete destroys a Dataproc cluster. | high |
| `bq-rm-recursive` | bq rm -r/-f removes BigQuery datasets, tables, or models — data lost. | critical |

### Allowlist Guidance

To allowlist a specific rule from this pack, add to your allowlist:

```toml
[[allow]]
rule = "cloud.gcp:<pattern-name>"
reason = "Your reason here"
```

To allowlist all rules from this pack (use with caution):

```toml
[[allow]]
rule = "cloud.gcp:*"
reason = "Your reason here"
risk_acknowledged = true
```

---

## Azure CLI

**Pack ID:** `cloud.azure`

Protects against destructive Azure CLI operations like vm delete, storage account delete, and resource group delete

### Keywords

Commands containing these keywords are checked against this pack:

- `az`
- `delete`
- `vm`
- `storage`
- `acr`
- `registry`
- `keyvault`
- `role`
- `ad`
- `dns`
- `cosmosdb`
- `monitor`
- `purge`

### Safe Patterns (Allowed)

These patterns match safe commands that are always allowed. Every rule allows Azure CLI global flags (`--subscription`, `--debug`, `--output`, `--verbose`, `--only-show-errors`) to appear between `az` and the subcommand. The subcommand literal is preceded by whitespace so flag values like `--account-name` or `--configure-files` cannot false-match a safe rule on a destructive command.

| Pattern Name | Pattern |
|--------------|----------|
| `az-show` | `az\b.*?\s+\S+\s+show` |
| `az-list` | `az\b.*?\s+\S+\s+list` |
| `az-account` | `az\b.*?\s+account\b` |
| `az-configure` | `az\b.*?\s+configure\b` |
| `az-login` | `az\b.*?\s+login\b` |
| `az-version` | `az\b.*?\s+version\b` |
| `az-help` | `az\b.*--help` |
| `az-what-if` | `az\b.*--what-if` |

### Destructive Patterns (Blocked)

These patterns match potentially destructive commands. Every rule matches with arbitrary global flags between `az` and the service name (`az --subscription prod vm delete …`, etc.).

| Pattern Name | Reason | Severity |
|--------------|--------|----------|
| `vm-delete` | az vm delete permanently destroys virtual machines. | critical |
| `storage-delete` | az storage account delete permanently destroys the storage account and all data. | critical |
| `blob-delete` | az storage blob/container delete permanently removes data. | high |
| `sql-delete` | az sql server/db delete permanently destroys the database. | critical |
| `group-delete` | az group delete removes the entire resource group and ALL resources within it! | critical |
| `aks-delete` | az aks delete removes the entire AKS cluster. | critical |
| `webapp-delete` | az webapp delete removes the App Service. | high |
| `functionapp-delete` | az functionapp delete removes the Azure Function App. | high |
| `cosmosdb-delete` | az cosmosdb delete permanently destroys the Cosmos DB resource. | critical |
| `keyvault-delete` | az keyvault delete removes the Key Vault. Secrets may be unrecoverable. | critical |
| `vnet-delete` | az network vnet delete removes the virtual network. | high |
| `acr-delete` | az acr delete removes the container registry and all images. | critical |
| `acr-repository-delete` | az acr repository delete permanently deletes the repository and its images. | high |
| `acr-repository-untag` | az acr repository untag removes tags from images. | medium |
| `keyvault-item-delete-or-purge` | Key Vault item delete/purge (az keyvault <key\|secret\|certificate\|storage> …) — purge bypasses soft-delete and is irreversible. | critical |
| `role-assignment-delete` | az role assignment delete removes an RBAC binding — users/SPs lose permissions. | high |
| `ad-sp-delete` | az ad sp delete removes a service principal — workloads using it lose auth. | critical |
| `ad-app-delete` | az ad app delete removes an Azure AD app registration — every service principal for it stops working. | critical |
| `network-dns-zone-delete` | az network dns zone delete removes an Azure DNS zone — domains stop resolving. | critical |
| `monitor-log-profiles-delete` | az monitor log-profiles delete removes a subscription activity-log export — compliance/forensics. | high |
| `cosmosdb-sql-container-delete` | az cosmosdb <api> <db\|container\|keyspace> delete permanently destroys Cosmos DB data. | critical |

### Allowlist Guidance

To allowlist a specific rule from this pack, add to your allowlist:

```toml
[[allow]]
rule = "cloud.azure:<pattern-name>"
reason = "Your reason here"
```

To allowlist all rules from this pack (use with caution):

```toml
[[allow]]
rule = "cloud.azure:*"
reason = "Your reason here"
risk_acknowledged = true
```

---


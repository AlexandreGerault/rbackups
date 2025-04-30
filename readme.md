# Work In Progress —— Automatic database backup

## What does it do?

The goal of this program is to easily orchestrate database backups. There are some cases where it is a good idea to save
one database to multiple storage, or even multiple databases to multiple storages. You might also want to automate it.
We also aim to integrate it easily with Docker, with cron jobs or with sysmtemd.

### But I can write a single script, and it actually does the job, cannot I?

If you believe you can actually write a script to actually fit your needs, feel free to do so, it might be a better
option for your use case. However, if you want to configure your backup with a single configuration file, then you're
currently looking at the right place.

### What would it look like?

We aim to provide a tool that requires minimal setup, with a single JSON option file. We'll consider adding other
options (like using a database configuration, that would be configurable through a web interface) later, as it doesn't
fit our own needs for now.

Here is an example we got after some thinking:

```json
{
  "connections": [
    {
      "id": "mysql-1",
      "driver": "mysql",
      "host": "mysql",
      "port": 3306,
      "username": "root",
      "password": "root"
    }
  ],
  "storages": [
    {
      "id": "aws",
      "driver": "s3",
      "access_key": "key",
      "secret_access": "secret",
      "region": "eu-west",
      "bucket": "backups"
    }
  ],
  "exports": [
    {
      "id": "laravel-data",
      "connection": "mysql-1",
      "database": "mei_laravel",
      "storage": "aws"
    }
  ]
}
```

As you can see, it's really easy to understand what is going on:

1. We define a list of connections. This is our data sources. It can be some MySQL databases, or some other SQL
   databases.
   We only aim for MySQL, Sqlite and PostgresSQL for the moment as they are needs we currently have.
2. We define a list of storages. These are our backup storage, where we will save the database backups. We aim, for now,
   to support S3 storage and local storage. We can consider later to add ftp and other kinds of upload protocol.
3. We define exports. These are our workflows to save databases to external (or local actually) storages. We select, for
   each export, a single data source and either a single or a list of storages to export it.

We think that this configuration file would cover our needs for every one of our projects. It supports the databases we
regularly use, it supports storages that really look relevant to save backups.

With this configuration file, we can imagine running it via a <abbr title="Command line interface">CLI</abbr> like so:

```shell
db-backup export --configuration=configuration.json
```

This command then can be used with a CRON job. We also think about creating a very light docker image so it could be
easily integrated in a Docker-based environment.

## The status of the project

As of today, the project is just an idea that is being shaped. We focus on writing documentation so it would drive our
implementation of the product.

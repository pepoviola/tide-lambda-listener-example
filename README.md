# Tide Lambda listener example

A little example serverless application, that you can deploy with the [sam cli](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/what-is-sam.html), using Rust as a custom runtime and building with [Tide](https://github.com/http-rs/tide) and [tide-lambda-listener](https://github.com/Fishrock123/tide-lambda-listener) and using a PostgreSQL database to store the `greetings`.

You check this [post](https://towardsdatascience.com/setting-up-a-postgresql-instance-on-the-cloud-4ec4cf168239) for a nice list of `free-tier` offerings for a pg db.

db schema:
```sql
CREATE TABLE "public"."tide-lambda-example-greetings" (
    "name" text NOT NULL,
    "created_at" timestamp with time zone,
    "modified_at" timestamp with time zone,
    PRIMARY KEY ("name")
);
```

To build and deploy your application for the first time, you will need first to complete the `DB_URL` in the `template.yml` file with your database url and then run the following in your shell:

```bash
sam build
sam deploy --guided
```

This will deploy two lambda functions:

#### PostHello

Save your greeting into the db

```bash
    $ curl -X POST -d '{"name":"joe"}'  https://<id>.execute-api.us-east-2.amazonaws.com/Prod/hello

    Hello joe, welcome to this tide lambda example.
```



#### GettHello

Retrive your greeting

```bash
curl  https://<id>.execute-api.us-east-2.amazonaws.com/Prod/hello/joe
Hi again joe
```

### Custom build

SAM allow us to build each funcntion using a `makefile`, we are using this file to file also to set the `file` to use in our `build.rs` file allowing to **only** compile the needed function endpoint and inject it into the `main` function of our application.

*This is an exploration and could be more efficient ways to conditional compile the code.*

### Compile to AWS lambda

You will need to target `x86_64-unknown-linux-musl`, you can check this [article](https://docs.aws.amazon.com/serverless-application-model/latest/developerguide/building-custom-runtimes.html) about custom runtimes.
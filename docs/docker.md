# Docker Setup

Follow these instructions to create a local development database.

1. Once Docker is installed, set up a MySQL Docker container. You may wish to change the
    username and password. If you do, save these credentials for configuring the 
    microservice.
    ```sh
    docker pull mysql:latest
    docker images # Sanity-check that MySQL was pulled
    docker run --name accounts -e MYSQL_ROOT_PASSWORD=password -p 3306:3306 -d mysql:latest 
    ```

> [!NOTE]
>
> If you get an error when running `docker` commands, the Docker daemon likely is not 
> running. You will need to start the daemon before `docker` commands will work. For 
> example, on MacOS, you can do this by launching the Docker Desktop app. The following 
> commands may also work:
>
> ```sh
> sudo systemctl start docker
> sudo gpasswd -a "${USER}" docker
> ```

2. Check that the container started:
    ```sh
    docker ps
    ```
    You should see output like:
    ```
    CONTAINER ID   IMAGE          COMMAND                  CREATED         STATUS         PORTS                               NAMES
    34a44ef739c9   mysql:latest   "docker-entrypoint.s…"   6 seconds ago   Up 5 seconds   0.0.0.0:3306->3306/tcp, 33060/tcp   accounts
    ```

> [!TIP]
>
> If you want to pause the database to conserve system resources, run:
> ```sh
> docker stop accounts
> ```
> To restart the database to access it again, run:
> ```sh
> docker start accounts
> ```

3. Access the container via a Bash session to configure the database:
    ```sh
    docker exec -it accounts bash
    ```

4. Enter MySQL's REPL. When prompted for a password, enter the password you configured:
    ```sh
    mysql -p
    ```

> [!WARNING]
>
> For security reasons, you should generally **not** enter the password directly on the 
> command line by specifying it after `-p`. If you run `mysql -p` without specifying a 
> password, you will be prompted for one. This allows you to avoid exposing the password, 
> e.g. in your shell's history.

5. Run the [database setup commands](/scripts/db-create.sql) to set up the database. 
    Ensure every command returned an OK response.

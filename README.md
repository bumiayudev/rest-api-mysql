## Dengan mengucapkan Bismillah Restful api ini dibangung menggunakan Framework Actix Web dari bahasa RUST dan Database MySql

Langkah yang perlu diperhatikan dalam pembuatan projek restful api ini diantaranya:
1. Kloning repository github terlebih dahulu
2. Siapkan dahulu databasenya, karena saya menggunakan docker untuk menjalankan database tersebut
    Wajib kalian perhatikan dalam membuat database mysql lewat docker
    1. Pertama pull dahulu image MySql
    ```
    docker pull mysql
    ```
    2. Kedua siapkan container MySql
    ```
    docker run --name mysql -e MYSQL_ROOT_PASSWORD=password -p 3306:3306 -d mysql
    ```
    3. Ketiga koneksi database
    ```
    docker exec -it mysql bash
    ```
    4. Keempat buatkan database baru dengan nama blog_db
    ```
    mysql -U root -p
    create database blog_db;
    use blog_db;
    ```
    5. Kelima buatkan tabel baru di dalam database blog_db
    ```
    create table posts(id integer primary key auto_increment, title varchar(200), content text);
    ```
3. Jalankan projeknya dengan perintah 
    ```
    cargo watch -x run
    ```
4. Testing api post bisa lewat curl, postman, insomia atau aplikasi yang lainnya

    1. Endpoint get all posts
        127.0.0.1:5050/api/posts => method : GET
    2. Endpoint get single post
        127.0.0.1:5050/api/posts/1 => method : GET
    3. Endpoint create a new post
        127.0.0.1:5050/api/posts => method : POST
    4. Endpoint update post
        127.0.0.1:5050/api/post/1 => method : PATCH
    5. Endpoint delete post
        127.0.0.1:5050/api/post/1 => method : DELETE


Selamat mencoba, semoga berhasil. Happy coding!.

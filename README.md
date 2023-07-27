# Rust HTTP Server to display filesystem

Rust HTTP server to access the filesystem

In case the server is bound to 0.0.0.0 (i.e. localhost) to port 8008 visiting the following address on the browser 
http://0.0.0.0:8080/<CLIENT_NAME>/documents/ should list all the contents of the documents folder including files and subdirectories, and all the clients connected
- If a file `foo.txt` is selected  in the `documents` directory sending the URL request http://0.0.0.0:8080/documents/foo.txt should download the file in the browser

- If a subdirectory od `documents`, for instance named `new_folder`, is selected sending the URL request http://0.0.0.0:8080/documents/new_folder all the contents fo the subdirecotry should be listed

- Rust handles the backed responsibilities only, (i.e.) listing all files and directories given the URL

- Connections to server must be authenticated 

- All connections should be displayed

- Every client connected to the server must have a name (not unique) and an IP address

- The Files and Directtory listed will be comprehensive of `last_modified_date` and `file_size`, and can be searched by words



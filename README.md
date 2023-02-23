![API Image](https://doc.lucidworks.com/assets/images/logos/datasources/rest-api-logo.png)

#How to use Employee API


Note: Replace "http://127.0.0.1:8080/" to  your address.

Note: For every request you have to provide USERNAME and PASSWORD in yor BasicAuth 

Note: For every request you have to provide APIT_Token in yor BearerAuth - `Currently worikng on this`
 

`All api only support GET request`


## API for accessing and  modifying employee data

* To get data of perticular employee use below api

   URL :
   ```
   http://127.0.0.1:8080/users/{username}
   ```

* To get data of all employee use below api

   URL :
   ```
   http://127.0.0.1:8080/users
   ```

* To add new employee use below api
   
   URL :
   ```
   http://127.0.0.1:8080/insertuser
   ```

   Required body :
   ```
   {
    "Username":"Mike@123",
    "Password":"Mike@123",
    "Employee_name":"Mike",
    "Employee_salary":10000,
    "Employee_designation":"Web3 developer"
   }
   ```

* To update the data of perticular employee use below api

   URL :
   ```
   http://127.0.0.1:8080/updateuser/{username}
   ```

   Required body :
   ```
   {
    "Employee_name":"Mike",
    "Employee_salary":10000,
    "Employee_designation":"Web3 developer"
   }
   ```


* To delete perticular employee use below api

   URL :
   ```
   http://127.0.0.1:8080/deleteuser/{username}
   ```


## API for get API access and for generate new API_Token 


* To create new account for accessing employee data

   URL :
   ```
   http://127.0.0.1:8080/api/newuser/{username}/{password}
   ```


* To generate new API_Token for accessing employee data

   URL :
   ```
   http://127.0.0.1:8080/api/generatenewtoekn/{username}/{password}
   ```
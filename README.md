How to use Employee API
-----------------------

Note: Replace "http://127.0.0.1:8080/" to  your address.

Note: For every request you have to provide USERNAME and PASSWORD in yor BasicAuth 



1. To get data of perticular employee use below api

   URL -> http://127.0.0.1:8080/users/{username}


1. To get data of all employee use below api

   URL -> http://127.0.0.1:8080/users


1. To add new employee use below api

   URL -> http://127.0.0.1:8080/insertuser
   
   Required body :

   {
    "Username":"Mike@123",
    "Password":"Mike@123",
    "Employee_name":"Mike",
    "Employee_salary":10000,
    "Employee_designation":"Web3 developer"
   }

1. To update the data of perticular employee use below api

   URL -> http://127.0.0.1:8080/updateuser/{username}

   Required body :
   
   {
    "Employee_name":"Mike",
    "Employee_salary":10000,
    "Employee_designation":"Web3 developer"
   }


1. To delete perticular employee use below api

   URL-> http://127.0.0.1:8080/deleteuser/{username}


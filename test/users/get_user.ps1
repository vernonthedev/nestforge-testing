<#
--------------------------------------
    Getting a user using user_id
--------------------------------------
#>
# GET USER
$id = 1
$get_user_uri = "http://localhost:3000/api/v2/users/$id"
Invoke-RestMethod -Uri $get_user_uri -Method Get
<#
--------------------------------------
    Creating a new user
--------------------------------------
#>
$create_user_uri = "http://localhost:3000/api/v1/users"
$create_user_body = @{
    name="samuel"
} | ConvertTo-Json

Invoke-RestMethod -Uri $create_user_uri -Method Post -ContentType "application/json" -Body $create_user_body

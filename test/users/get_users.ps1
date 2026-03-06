<#
--------------------------------------
    Getting all users
--------------------------------------
#>
Write-Host "[+] Getting All Users.." -ForegroundColor Cyan -BackgroundColor DarkGreen
$get_users_uri = "http://localhost:3000/api/v2/users"
Invoke-RestMethod -Uri $get_users_uri -Method Get
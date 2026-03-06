<#
--------------------------------------
    Deleting a user using user_id
--------------------------------------
#>
Write-Host "[+] Deleting User.." -ForegroundColor Cyan -BackgroundColor DarkGreen
$delete_user_id = 1
$delete_user_uri = "http://localhost:3000/api/v2/users/$delete_user_id"
Invoke-RestMethod -Uri $delete_user_uri -Method Delete
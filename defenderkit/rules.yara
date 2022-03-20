/*
   YARA Rule Set
   Author: Michael Taggart <mtaggart@taggart-tech.com>
   Date: 2022-02-26
   Identifier: offnote
   Reference: https://github.com/Neo23x0/yarGen
*/

/* Rule Set ----------------------------------------------------------------- */

rule offensive_notion_linux {
   meta:
      description = "offnote - file offensive_notion"
      author = "Michael Taggart <mtaggart@taggart-tech.com>"
      reference = "https://github.com/Neo23x0/yarGen"
      date = "2022-02-26"
   strings:
      $x1 = "truecontentidresultsto_docheckedsrc/cmd/download.rs Downloading from Could not write file bytes written to /src/cmd/elevate.rsNe" ascii
      $x2 = "GOAWAY stream IDs shouldn't be higher; last_processed_id = , f.last_stream_id() = /home/mttaggart/.cargo/registry/src/github.com" ascii
      $x3 = "trailersHttpInfox-frame-optionsx-dns-prefetch-controlx-content-type-optionswarningviaupgrade-insecure-requestsupgradeuser-agentt" ascii
      $x4 = "attempt to divide by zeroInvalid ELF section name offsetInvalid ELF section size or offset/rustc/9d1b2106e23b1abd32fce1f17267604" ascii
      $x5 = " <unknown status code>InvalidStatusCodeinvalid status codedescription() is deprecated; use DisplayNetwork Authentication Require" ascii
      $x6 = " - UnsupportedNotUnicodeNotPresenterrorCustomUncategorizedOtherOutOfMemoryUnexpectedEofInterruptedArgumentListTooLongFilenameToo" ascii
      $x7 = "PathslabheadtailSlotnextDELETEPUTGETOPTIONS1001011021031041051061071081091101111121131141151161171181191201211221231241251261271" ascii
      $x8 = "Concurrency value must be greater than 0!src/cmd/ps.rssrc/cmd/pwd.rsRunas only works on Windows!src/cmd/save.rsConfig file saved" ascii
   condition:
      uint16(0) == 0x457f and filesize < 16000KB and
      1 of ($x*)
}

rule offensive_notion_windows {
   meta:
      description = "offnote - file offensive_notion.exe"
      author = "Michael Taggart <mtaggart@taggart-tech.com>"
      reference = "https://github.com/Neo23x0/yarGen"
      date = "2022-02-26"
   strings:
      $x1 = " /create /tn Notion /tr \"C:\\Windows\\System32\\cmd.exe ' -c '\" /sc onlogon /ru System\"" fullword ascii
      $x2 = "Elevation attempted. Look for the new agent!" ascii
      $x3 = "Elevation unavailable" ascii
      $x4 = "$FilterArgs = @{ name='Notion';EventNameSpace='root\\CimV2';QueryLanguage=\"WQL\"; Query=\"SELECT * FROM __InstanceModificationE" ascii
      $x5 = "Could not parse command!" ascii
      $x6 = "cddownloadelevategetprivsinjectpersistportscanpspwdrunassaveshellshutdownsleep" ascii
   condition:
      uint16(0) == 0x5a4d and filesize < 21000KB and
      1 of ($x*)
}


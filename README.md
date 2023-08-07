# notion-rustport
Create daily reports automatically from your Notion task page. Project aimed to reduce the workload when making the daily work report.
This project achieves the same goal as [DailyReportCreator](https://github.com/ErlantzCalvo/DailyReportCreator), but this one is made using Rust.

Disclaimer: If you are a rustacean, feel free to insult me for the poor quality of the code.
# Example
Having the following Notion Scrum-like ToDo page
![Notion To do page](https://github.com/ErlantzCalvo/DailyReportCreator/blob/main/media/notion_example.png?raw=true)

This project creates the following report:

<img src="https://github.com/ErlantzCalvo/DailyReportCreator/blob/main/media/output_example.png?raw=true" alt="Generated report" width="700"/>

# Usage
You can download the binaries in the [Releases](https://github.com/ErlantzCalvo/notion-rustport/releases) section or install it manually following the [Installation section](https://github.com/ErlantzCalvo/DailyReportCreator#installation).

Once you have downloaded the binaries, you have to fill the <i>.env</i> file with your Notion settings ([Setup section](https://github.com/ErlantzCalvo/DailyReportCreator#setup)) and modify the config.json file in order to change the output format if wanted.

Once these things are done, and with the right permissions, run the binary as follows:
`./notion-rustport`

<b>Note:</b> You can check the usage using `./notion-rustport -h` or `./notion-rustport --help`;

# Installation
Clone the repo:<br>
`git clone https://github.com/ErlantzCalvo/notion-rustport`

Place in the folder: <br>
`cd notion-rustport`

Install the dependencies:<br>
`cargo run`

# Setup
In order to run the project, you must have a Notion [API key.](https://www.notion.so/my-integrations) If you don't know how to create the mentioned key, take a look at their [well explained documentation](https://developers.notion.com/docs/getting-started).

Once you have the API key, add it to the `.env` file located in the project's root folder, replacing the field *<API_KEY>* by your key. The next step is to get the ID of the page you want to track/be reported. It is also explained in the documentiation but, in short, if you are using Notion in the browser, the page ID is the string located between <workpace name>/<Page ID>?v=...:

```
  https://www.notion.so/myworkspace/a8aec43384f447ed84390e8e42c2e089?v=...
                                    |--------- Database ID ---------|
```
If you are using the Notion desktop app, you can get the previous link in the top-right part of it, in the share button -> Copy link.
Once you have the ID of the page you want to track, place it in the `.env` file, replacing the _\<Page ID\>_ field. 
  
***Note:*** Remember to give your API key, at least, read acces of the page you want to track as shown in the [documentation](https://developers.notion.com/docs/getting-started#step-1-create-an-integration)
  
 
### Options
-c or --to-clipboard : Copy the resulting report to the clipboard. <br>
--config_path : Specify the config.json file location. <br>
-h or --help : Display the available options.
  
# Configuration
The app need to know which is the title for each status. This is, in the picture of the notion page (At the top of this README) it can be seen that the 3 status names are *To Do, Doing* and *Done 🙌*. This three names must be put in the `config.json` file:
```
  "TasksStatus": {
        "PendingTasks":"To Do",
        "DoingTasks":"Doing",
        "FinishedTasks": "Done 🙌"
    }```

  It can also be configured the texts that will be displayed in the output in the *Texts* field:
  ```
  "Texts": {
        "CurrentStatusFinished": "FINISHED",
        "CurrentStatusDoing": "IN PROGRESS",
        "BeginningOfMessage": "My daily report today is as follows:\n",
        "PendingTasksBeginning": "\nPending tasks:\n"
    }
  ```

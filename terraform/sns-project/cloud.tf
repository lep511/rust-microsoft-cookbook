terraform { 
  cloud { 
    
    organization = "new-workshop-data" 

    workspaces { 
      name = "SNS-Test" 
    } 
  } 
}
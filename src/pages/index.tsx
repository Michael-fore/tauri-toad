import { useState} from "react";
import { invoke } from "@tauri-apps/api/tauri";
import SqlEditor from '../components/editor'
import MUIDataTable from "mui-datatables";
import React from 'react';

const columns = ["Name", "Company", "City", "State"];

const data = [
 ["Joe James", "Test Corp", "Yonkers", "NY"],
 ["John Walsh", "Test Corp", "Hartford", "CT"],
 ["Bob Herm", "Test Corp", "Tampa", "FL"],
 ["James Houston", "Test Corp", "Dallas", "TX"],
];

const options = {
  filterType: 'checkbox',
};

//meant to mirror the type is runst
interface Row{
  values:Array<String>
}

interface Column{
  value:String,
  datatype:String
}

interface Return{
  columns:Array<Column>,
  rows:Array<Row>,
}



function App() {
  const [sqlInterfase,   setSqlInterfase] = useState(<></>);
  const [selectionState, setSelectionState] = useState();
  const [editorState, setEditorState] = useState();

  const setError = (error:String)=>{
    setSqlInterfase(<div>
      <div>{error}</div>
    </div>);
  }

  async function execute() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    // console.log(editorState, selectionState);
    var error:boolean = false;
    const sql = editorState;
    const data = await invoke("execute", { sql })
    console.log(data);
    const columns = data.columns.map((c)=>{if(c.datatype=='ERROR'){error=true};return c.value})
    const values = data.rows.map((row)=>{return row.values})

    if (error){
      setError(columns[0]);
    } else if(columns.length==0){
      setSqlInterfase(<></>);
    } else {
      setSqlInterfase( 
      <MUIDataTable
        data={values}
        columns={columns}
        draggableColumns={true}
      />);
    } 
  }

  const handle_hot_keys = (e) => {
    if(e.code=="F9"){
      execute()
    }
  }

  return (<>
    <div className="container" onKeyDown={(e)=>{handle_hot_keys(e)}}>
    <button className="r-0 h-8 w-16 bg-blue-400 rounded-md" type="button" onClick={() => execute()} >
              Execute
    </button>
  <SqlEditor  setSelectionState={setSelectionState} setEditorState={setEditorState}/>
        
        <div className="w-screen">{sqlInterfase}</div>
       
      </div>
      </>
  );
}

export default App;

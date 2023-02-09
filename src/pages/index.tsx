import { useState, useEffect} from "react";
import { invoke } from "@tauri-apps/api/tauri";
import SqlEditor from '../components/editor'
import MUIDataTable from "mui-datatables";
import React from 'react';
import DBForm from '../components/db_credentials';
import { emit, listen } from '@tauri-apps/api/event'
import { useRouter } from "next/router";
import PlayArrowIcon from '@mui/icons-material/PlayArrow';

//meant to mirror the type is rust
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
  const router = useRouter();

  const unlisten  = async () => {
    await listen('connect', (event) => {
      router.push(event.payload);
    })
  }
  
  useEffect(()=>{
    unlisten().then((e)=>{console.log(e)})
  },[])

  const setError = (error:String)=>{
    setSqlInterfase(<div>
      <div>{error}</div>
    </div>);
  }

  const test_storage = () =>{
    localStorage.setItem("key", JSON.stringify('{"label":"test"}'));
    console.log(JSON.parse(localStorage.getItem("key")));
  }

  async function execute() {
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    // console.log(editorState, selectionState);
    var error:boolean = false;
    const sql = editorState;
    const data = await invoke("execute", { sql })
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

  return (
    <div className="container" onKeyDown={(e)=>{handle_hot_keys(e)}}>
    <button 
      className="h-8 w-8 hover:bg-slate-100 active:bg-slate-200" 
      type="button" 
      onClick={() => execute()} >
        <PlayArrowIcon className="text-green-400"/>
    </button>
    <SqlEditor  
      setSelectionState={setSelectionState} 
      setEditorState={setEditorState}/>
        <div className="b-1"></div>
    <div className="w-screen h-auto">{sqlInterfase}</div>
       
    </div>
  );
}

export default App;

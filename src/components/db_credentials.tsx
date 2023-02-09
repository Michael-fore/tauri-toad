
import TextField from '@mui/material/TextField';
import Stack from '@mui/material/Stack';
import {useState} from 'react';
import { useRouter } from 'next/router';
import { invoke } from "@tauri-apps/api/tauri";
import InputLabel from '@mui/material/InputLabel';
import MenuItem from '@mui/material/MenuItem';
import FormControl from '@mui/material/FormControl';
import Select, { SelectChangeEvent } from '@mui/material/Select';

const driver_list = ['mysql','oracle']

function DriverDropdown({driver, setDriver}){
 return <FormControl fullWidth>
    <InputLabel id="demo-simple-select-label">Sql Driver</InputLabel>
        <Select
          label="Sql Driver"
          value={driver}
          onChange={(e)=>{setDriver(e.target.value)}}
        >
          {
            driver_list.map((d)=>{
                return <MenuItem value={d}>{d}</MenuItem>
            })
        }
        </Select>
 </FormControl>
}

export default function DBForm(){
    const [driver, setDriver] = useState("mysql"); //will be a sql dropdown at somepoint
    const [url, setUrl] = useState("10.200.11.141:3306");
    const [username, setUsername] = useState("test");
    const [password, setPassword] = useState("password");
    const [database, setDatabase] = useState("test");
    const [message, setMessage] = useState({});
    const router = useRouter();

    const test_connection = async () => {
        const data = await invoke("test_connection", 
            { driver:driver, url:url, username:username , password:password , database:database }) 
        setMessage(data);
    }

    const connect = async () => {
        const data = await invoke("connect", 
            { driver:driver, url:url, username:username , password:password , database:database })

        if(data=="Success"){
            router.push('../')
        }
        setMessage(data);
    }

    const back = () =>{
        //kinda dumb, should be passing state areound so user can go as 
        // deep as they want
        router.push('../')
    }

    return (
        <>
            <div className='w-64'>
            <DriverDropdown driver={driver} setDriver={setDriver}/>
            <Stack>
                <TextField label="Url" variant="standard" 
                    value={url}
                    onChange={(e)=>{setUrl(e.target.value)}}/>
                <TextField label="Username" variant="standard" 
                    value={username}
                    onChange={(e)=>{setUsername(e.target.value)}} />
                <TextField label="Password" variant="standard"
                    value={password}
                    type="password"
                    onChange={(e)=>{setPassword(e.target.value)}}/>
                <TextField label="Default Database" variant="standard" 
                    value={database}
                    onChange={(e)=>{setDatabase(e.target.value)}}/>
            </Stack>
            </div>
        <button className='w-32 h-8 bg-blue-400 rounded-xl'
            onClick={()=>{test_connection()}}>
            Test Connection
        </button>
        <button className='w-24 h-8 bg-blue-400 rounded-xl'
            onClick={()=>{connect()}}>
            Connect
        </button>
        <button className='w-20 h-8 bg-blue-400 rounded-xl'
            onClick={()=>{back()}}>
            Back
        </button>
        
        </>
    );
      
}
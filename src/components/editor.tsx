
import CodeMirror from '@uiw/react-codemirror';
import { sql } from '@codemirror/lang-sql';
import {useCallback} from 'react';
import { EditorView, ViewUpdate } from '@codemirror/view';
import { EditorSelection, SelectionRange } from '@codemirror/state';
import { historyField } from '@codemirror/commands';

export default function SqlEditor({setSelectionState,setEditorState}){

  const onChange = useCallback((value:string, viewUpdate:ViewUpdate):void => {
        console.log('values:', value.replace('\n',''));
        console.log(viewUpdate.state.selection.main);
        setSelectionState(viewUpdate.state.selection);
        setEditorState(value);
    }, []);

    return <div className='w-screen'>
            <CodeMirror
                value=""
                height="200px"
                extensions={[sql()]}
                onChange={onChange}
             >
             </CodeMirror>
        </div>
}
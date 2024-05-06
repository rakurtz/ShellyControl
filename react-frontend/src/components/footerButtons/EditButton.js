import './EditButton.css';

// Material UI
import { Fab } from '@mui/material';
import SettingsOutlinedIcon from '@mui/icons-material/SettingsOutlined';
import SaveOutlinedIcon from '@mui/icons-material/SaveOutlined';
import CancelOutlinedIcon from '@mui/icons-material/CancelOutlined';

// global import
// import { backendUrl } from '../App';

export default function EditButton(props) {
    // const allLampsAreOff = props.lamps.every(lamp => lamp.is_on === false);
    const editMode = props.editMode;
    const setEditMode = props.setEditMode;
    const setSaveCommand = props.setSaveCommand;
    const setForceReload = props.setForceReload;
    
    const handleEdit = (e) => {
        e.preventDefault();
        setEditMode(true);
    };

    const handleSave = (e) => {
        e.preventDefault();
        setSaveCommand(true);
        setTimeout(() => {          // need short delay here to let EditContainer realize state change of saveCommand
            setEditMode(false);
            setForceReload(true);
        }, 700);
    };

    const handleCancel = (e) => {
        e.preventDefault();
        setEditMode(false);
    };

    const edit = (
        <Fab color="secondary" aria-label="editMode">
            <SettingsOutlinedIcon fontSize='large' onMouseUp={handleEdit} onTouchEnd={handleEdit}/>
        </Fab>
        );

    const save_or_cancel = ( 
        <div className='save_or_cancel_inner'>
            <Fab color="secondary" aria-label="editMode" onMouseUp={handleSave} onTouchEnd={handleSave}>
                <SaveOutlinedIcon fontSize='large'/>
            </Fab>
            <Fab color="secondary" aria-label="editMode" onMouseUp={handleCancel} onTouchEnd={handleCancel}>
                <CancelOutlinedIcon fontSize='large'/>
            </Fab>
        </div>
    
        );

    
    // css classes 
    return (
        <div className='save_or_cancel_relative'>
            <div>
                { editMode ? save_or_cancel : edit }
            </div>
        </div>
     );
  }
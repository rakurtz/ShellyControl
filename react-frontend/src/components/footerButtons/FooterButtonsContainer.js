import './FooterButtonsContainer.css';

import ForceReload from './ForceReload';
import EditButton from './EditButton';
import GlobalSwitch from './GlobalSwitch';

export default function FooterButtonsContainer(props) {
    const lamps = props.lamps;
    const setLamps = props.setLamps;
    const editMode = props.editMode;
    const setEditMode = props.setEditMode;
    const setForceReload = props.setForceReload;
    const setSaveCommand = props.setSaveCommand;


    const force_reload_jsx = <ForceReload setForceReload={setForceReload}/>;
    const global_swith_jsx = <GlobalSwitch lamps={lamps} setLamps={setLamps}/>;
    return (
        <div className='bottom_fabs'>
            <EditButton editMode={editMode} setEditMode={setEditMode} setSaveCommand={setSaveCommand} setForceReload={setForceReload}/>
            <div className='bottom_fabs_grp'>
                {!editMode && force_reload_jsx}
                {!editMode && global_swith_jsx}
            </div>
        </div>
    )
}
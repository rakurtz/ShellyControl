import './EditShellyLamp.css';
import { isValidIPAddress } from '../../helpers/isValidIPAddr';

import { Button } from '@mui/material';
import Box from '@mui/material/Box';

import { useEffect, useState } from 'react';


export default function EditShelly(props) {
   // props
    const shellies = props.shellies;
    const setShellies = props.setShellies;
    const handleShellyModalClose = props.handleShellyModalClose;
    const editShellyWithId = props.editShellyWithId; // when creating a new on, just give next unused id

    // standard light's array (only used when saving config to backend)
    const standardLights = [
        {"state":"Off","brightness":50},
        {"state":"Off","brightness":50},
        {"state":"Off","brightness":50},
        {"state":"Off","brightness":50}
    ]

    // determing if new or existing shelly
    const [modifiedShellyDeviceName, setModifiedShellyDeviceName] = useState("");
    const [modifiedShellyIp, setModifiedShellyIp] = useState("");
    const existingShelly = shellies.find(shelly => shelly.id === editShellyWithId);   
    useEffect(() => {
        if (existingShelly) {
            setModifiedShellyDeviceName(existingShelly.device_name);
            setModifiedShellyIp(existingShelly.ip);
        }
    }, [editShellyWithId]) // (only once)


    // Function to update the shellies with the changed element
    const updateConfigShellies = (modifiedShelly) => {
        // Find the index of the element you want to change
        const index = shellies.findIndex(obj => obj.id === editShellyWithId);
        console.log(index)
        var updatedShellies = [];
        if (index !== -1) { // If the element is found
            // Create a new array with the changed element
            updatedShellies = [
                ...shellies.slice(0, index), // Elements before the changed one
                modifiedShelly, // The changed element
                ...shellies.slice(index + 1) // Elements after the changed one
            ];

            // Update the state with the new array
            setShellies(updatedShellies);
        } else {
            // adding new shelly
            updatedShellies = shellies
            updatedShellies.push(modifiedShelly);
            setShellies(updatedShellies);
        }
    };


    const handleClickOk = () => {
        // implementing logic of adding / patchin modifiedShelly in configShellies
        const modifiedShelly = {
            id: editShellyWithId,
            device_name: modifiedShellyDeviceName,
            ip: modifiedShellyIp,
            lights: standardLights,
        }
        updateConfigShellies(modifiedShelly)
        setTimeout(() => {          // need short delay here to let EditContainer realize state change of saveCommand
            handleShellyModalClose()
        }, 200);
    }


    // from MUI/Modal (https://mui.com/material-ui/react-modal/)
    const style = {
        position: 'absolute',
        top: '50%',
        left: '50%',
        transform: 'translate(-50%, -50%)',
        width: '80%',
        maxWidth: '400px',
        bgcolor: 'background.paper',
        border: '2px solid #000',
        boxShadow: 24,
        p: 4,
      };

    return (
        <Box sx={style}>
            <h3>Shelly with ID: {editShellyWithId}</h3>
            <table className='edit-table'>
                <tbody>
                <tr>
                    <td>
                        Device Name:
                    </td>
                    <td>
                        <input className="input-field" name="deviceName" defaultValue={modifiedShellyDeviceName} onChange={(e) => setModifiedShellyDeviceName(e.target.value.trim())}/>
                    </td>
                </tr>
                <tr>
                    <td>
                        IP-Address: 
                    </td>
                    <td>
                        <input className="input-field" name="ip" defaultValue={modifiedShellyIp} onChange={(e) => setModifiedShellyIp(e.target.value.trim())}/>
                    </td>
                </tr>
                </tbody>
            </table>
                        <Button variant="outlined" onClick={handleShellyModalClose}>Cancel</Button> &nbsp;
                        <Button variant="contained" onClick={handleClickOk} disabled={!(isValidIPAddress(modifiedShellyIp) && modifiedShellyDeviceName.length > 0)}>Ok</Button>
        </Box>
    )
}
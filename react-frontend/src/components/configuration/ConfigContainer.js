// EditContainer
//
// this component stores the logic of editing the available shellies and lamps
// it retrieves the available shellies and lamps from the backend via /get_config
//
// on editing the local respresentation of lamps and shellies are altered (in the useState())
// on save the local state gets sent to the backend
// on cancel we just return to the normal view and retrieve lamps from backend again

import "./ConfigContainer.css";

// mui
import Modal from "@mui/material/Modal";
import EditIcon from "@mui/icons-material/Edit";
import DeleteForeverOutlinedIcon from "@mui/icons-material/DeleteForeverOutlined";

import { backendUrl } from "../../App";
import { useEffect, useState } from "react";
import EditShelly from "./ConfigEditShelly";
import EditLamp from "./ConfigEditLamp";
import { Button } from "@mui/material";

export default function LampContainer(props) {
  // props
  const saveCommand = props.saveCommand; // get's triggered by EditButton Component
  const setSaveCommand = props.setSaveCommand;

  // state
  const [configShellies, setConfigShellies] = useState([]);
  const [configLamps, setConfigLamps] = useState([]);
  const [showShellyModal, setShowShellyModal] = useState(false);
  const [editShellyWithId, setEditShellyWithId] = useState(0);
  const [showLampModal, setShowLampModal] = useState(false);
  const [editLampWithId, setEditLampWithId] = useState(0);

  //
  // fetch data
  // getting config from backend and fill state
  useEffect(() => {
    console.log("INFO: getting /get_config");
    fetch(backendUrl + "/config")
      .then((res) => res.json())
      .then((data) => {
        setConfigShellies(data.shellies);
        setConfigLamps(data.lamps);
        // console.log(shellies);
        console.log(data.lamps);
      })
      .catch((error) => console.log("Error fetching data:", error));
  }, []);

  //
  // functions
  // save
  useEffect(() => {
    console.log("INFO: saveCommand state changed to: " + saveCommand);
    if (saveCommand) {
      console.log("... simulated api post request with actual config");
      const requestOptions = {
        method: "POST",
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ shellies: configShellies, lamps: configLamps }),
      };
      fetch(backendUrl + "/config", requestOptions).then((response) =>
        console.log(response)
      );

      setSaveCommand(false);
    }
  }, [saveCommand]);

  // map lamp member ids to shelly names
  const members_add_shelly_name = (members) => {
    var named_members = [];
    members.forEach((member) => {
      const shelly = configShellies.find(
        (shelly) => shelly.id === member.device_id
      );
      if (shelly) {
        named_members.push({
          device_id: member.device_id,
          device_name: shelly.device_name,
          lane: member.lane,
        });
      }
    });
    return named_members; // works!
  };

  const members_to_jsx = (members) => {
    const named_members = members_add_shelly_name(members);
    const members_li_jsx = named_members.map((member) => (
      <li key={member.device_id + "-" + member.lane}>
        {member.device_name}: {member.lane}
      </li>
    ));
    return <ul className="members_ul">{members_li_jsx}</ul>;
  };

  // edit shelly modal
  const handleShellyModalOpen = () => {
    setShowShellyModal(true);
  };
  const handleShellyModalClose = () => {
    setShowShellyModal(false);
  };
  const handleEditShellyWithId = (id) => {
    setEditShellyWithId(id);
    handleShellyModalOpen();
  };
  const getNewShellyId = () => {
    var id = 0;
    configShellies.forEach((shelly) => {
      if (shelly.id >= id) {
        id = shelly.id;
      }
    });
    id = id + 1;
    return id;
  };

  const handleDeleteShellyClickWithID = (id) => {
    if (window.confirm(`Do you want to delete Shelly with ID ${id}?`)) {
      deleteShellyWithID(id);
    }
  };

  const deleteShellyWithID = (id) => {
    setConfigShellies((prevConfigShellies) =>
      prevConfigShellies.filter((shelly) => shelly.id !== id)
    );
  };

  // edit lamp modal
  const handleLampModalOpen = () => {
    setShowLampModal(true);
  };
  const handleLampModalClose = () => {
    setShowLampModal(false);
  };
  const handleEditLampWithId = (id) => {
    setEditLampWithId(id);
    handleLampModalOpen();
  };
  const getNewLampId = () => {
    var id = 0;
    configLamps.forEach((lamp) => {
      if (lamp.id >= id) {
        id = lamp.id;
      }
    });
    id = id + 1;
    return id;
  };

  const handleDeleteLampClickWithID = (id) => {
    if (window.confirm(`Do you want to delete Lamp with ID ${id}?`)) {
      deleteLampWithID(id);
    }
  };

  const deleteLampWithID = (id) => {
    setConfigLamps((prevConfigLamps) =>
      prevConfigLamps.filter((lamp) => lamp.id !== id)
    );
  };

  //
  // jsx
  //
  const shelly_table_rows_jsx = configShellies.map((shelly) => (
    <tr key={shelly.id}>
      <td>{shelly.id}</td>
      <td>{shelly.device_name}</td>
      <td>{shelly.ip}</td>
      <td>
        <EditIcon onClick={() => handleEditShellyWithId(shelly.id)} />
        &nbsp;
        <DeleteForeverOutlinedIcon
          onClick={() => handleDeleteShellyClickWithID(shelly.id)}
        />
      </td>
    </tr>
  ));

  const shelly_table_jsx = (
    <table className="custom-table" key="shelly-table">
      <thead>
        <tr>
          <th colSpan="4">
            <h3>Shellies</h3>
          </th>
        </tr>
        <tr>
          <th>ID</th>
          <th>Device Name</th>
          <th>IP-Address</th>
          <th>Modify</th>
        </tr>
      </thead>
      <tbody>
        {shelly_table_rows_jsx}
        <tr>
          <td colSpan="4">
            <Button
              variant="contained"
              onClick={() => handleEditShellyWithId(getNewShellyId())}
            >
              Add Shelly
            </Button>
          </td>
        </tr>
      </tbody>
    </table>
  );

  const lamps_table_rows_jsx = configLamps.map((lamp) => (
    <tr key={lamp.id}>
      <td>{lamp.id}</td>
      <td>{lamp.name}</td>
      <td>{members_to_jsx(lamp.members)}</td>
      <td>
        <EditIcon onClick={() => handleEditLampWithId(lamp.id)} />
        &nbsp;
        <DeleteForeverOutlinedIcon
          onClick={() => handleDeleteLampClickWithID(lamp.id)}
        />
      </td>
    </tr>
  ));

  const lamps_table_jsx = (
    <table className="custom-table" key="lamp-table">
      <thead>
        <tr>
          <th colSpan="4">
            <h3>Lamps</h3>
          </th>
        </tr>
        <tr>
          <th>ID</th>
          <th>Name</th>
          <th>Shelly-Lanes</th>
          <th>Modify</th>
        </tr>
      </thead>
      <tbody>
        {lamps_table_rows_jsx}
        <tr>
          <td colSpan="4">
            <Button
              variant="contained"
              onClick={() => handleEditLampWithId(getNewLampId())}
            >
              Add Lamp
            </Button>
          </td>
        </tr>
      </tbody>
    </table>
  );

  return (
    <>
      <h2>Configuration</h2>
      <div className="flex_container">{shelly_table_jsx}</div>
      <div className="flex_container">{lamps_table_jsx}</div>
      <div className="bottom_space"></div>

      <Modal
        open={showShellyModal}
        onClose={handleShellyModalClose}
        aria-labelledby="modal-shelly-edit"
        aria-describedby="edit or add a shelly by id"
      >
        <EditShelly
          shellies={configShellies}
          setShellies={setConfigShellies}
          editShellyWithId={editShellyWithId}
          handleShellyModalClose={handleShellyModalClose}
        />
      </Modal>

      <Modal
        open={showLampModal}
        onClose={handleLampModalClose}
        aria-labelledby="modal-lamp-edit"
        aria-describedby="edit or add a lamp by id"
      >
        <EditLamp
          lamps={configLamps}
          setLamps={setConfigLamps}
          editLampWithId={editLampWithId}
          shellies={configShellies}
          handleLampModalClose={handleLampModalClose}
        />
      </Modal>
    </>
  );
}

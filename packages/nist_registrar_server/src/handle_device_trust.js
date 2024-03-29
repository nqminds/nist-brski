export default async function handleDeviceTrust(claimData, dbGet, dbRun) {
  const {user, device, trust, issuanceDate} = claimData;
  let deviceId = null;
  const deviceRow = await dbGet("SELECT id from device where id = ? OR name = ?", [device, device])
  if (!deviceRow) {
    return `No device with id or name ${device}`;
  } else {
    deviceId = deviceRow.id;
  }
  let userId = null;
  const userRow = await dbGet("SELECT id from user where id = ? OR username = ?", [user, user])
  if (!userRow) {
    return `No user with id or name ${user}`;
  } else {
    userId = userRow.id;
  }
  const trustRow = await dbGet("SELECT * from allow_to_connect WHERE authoriser_id = ? AND device_id = ?", [userId, deviceId])
  if (trust) {
    if (!trustRow) {
      await dbRun("INSERT INTO allow_to_connect (authoriser_id, device_id) VALUES (?, ?)",
        [userId, deviceId]);
      return `Trust added to device ${device} by user ${user}`;
    } else {
      return `device ${device} is already trusted by user ${user}`;
    }
  } else {
    if (!trustRow) {
      return `device ${device} is not trusted by user ${user}`;
    } else {
      await dbRun("DELETE FROM allow_to_connect WHERE authoriser_id = ? AND device_id = ?", [userId, deviceId]);
      return `Trust removed from device ${device} by user ${user}`;
    }
  }
}

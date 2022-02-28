<?php 
$config = json_decode(file_get_contents("/etc/discordrpc-server/config.json"));
$albums = glob($config->root."/*");
function getOriginURI() {
    $uri_paths = explode('?', $_SERVER['REQUEST_URI'], 2);
    return $_SERVER['REQUEST_SCHEME'] . "://" . $_SERVER[HTTP_HOST] . $uri_paths[0];
}
$output = array(
    "name" => $config->vendorName,
    "uuid" => $config->uuid,
    "hosticon" => $config->hosticon,
    "secured" => $config->passwordLocked,
    "root" => $config->CDNroot,
    "originURI" => getOriginURI(),
    "albums" => array(),
    "size" => 0
);
foreach ($albums as $album) {
    $folderName = str_replace($config->root."/", "", $album);
    $absoluteDir = $config->root."/".$folderName;
    $metadata = json_decode(file_get_contents($absoluteDir."/.metadata.json"));
    $allFiles = glob($absoluteDir."/*");
    $musicFiles = array();
    foreach ($allFiles as $path) {
        $audioSize = filesize($path);
        $output["size"] += $audioSize;
        array_push(
            $musicFiles,
            $fileEntry = array(
                "name" => str_replace($absoluteDir."/", "", $path),
                "size" => $audioSize
            )
        );
    }
    $albumObject = array(
        "dir" => $folderName,
        "name" => $metadata->name,
        "cover" => $metadata->cover,
        "files" => $musicFiles
    );
    array_push($output['albums'], $albumObject);
}
$result = json_encode($output, JSON_PRETTY_PRINT);
header("Content-Type: application/json");
header("Content-Length: ".strlen($result));
echo $result;
?>
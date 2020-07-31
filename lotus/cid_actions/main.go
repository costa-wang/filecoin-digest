package main

import (
	"fmt"
	 cid "github.com/ipfs/go-cid"
)

var Cidstr string = "zb2rhe5P4gXftAwvA4eXQ5HJwsER2owDyS9sKaQRRVQPn93bA"

func main() {
    eecid, err0 := cid.ExtractEncoding(string(Cidstr))
	if err0 != nil {
	     fmt.Println(eecid) 
		 } else {
		 fmt.Println(eecid) // 122
		 }
	var CidIns cid.Cid
	parsed := CidIns.Type()
	fmt.Println(parsed) 
}
import { useState } from "react";
import { SearchBar } from "../SearchBar";

export default function SearchBarExample() {
  const [searchValue, setSearchValue] = useState("");

  return (
    <SearchBar
      value={searchValue}
      onChange={setSearchValue}
      onSearch={(val) => console.log("Searching:", val)}
    />
  );
}

// Code generated by github.com/actgardner/gogen-avro/v10. DO NOT EDIT.
/*
 * SOURCE:
 *     avro.json
 */
package avro

import (
	"encoding/json"
	"fmt"
	"io"

	"github.com/actgardner/gogen-avro/v10/compiler"
	"github.com/actgardner/gogen-avro/v10/vm"
	"github.com/actgardner/gogen-avro/v10/vm/types"
)

var _ = fmt.Printf

type User struct {
	Id string `json:"id"`

	Name string `json:"name"`

	Username string `json:"username"`

	Created_at string `json:"created_at"`

	Followers int64 `json:"followers"`
}

const UserAvroCRC64Fingerprint = "<\x85q\x16\xf4U\x17\xcd"

func NewUser() User {
	r := User{}
	return r
}

func DeserializeUser(r io.Reader) (User, error) {
	t := NewUser()
	deser, err := compiler.CompileSchemaBytes([]byte(t.Schema()), []byte(t.Schema()))
	if err != nil {
		return t, err
	}

	err = vm.Eval(r, deser, &t)
	return t, err
}

func DeserializeUserFromSchema(r io.Reader, schema string) (User, error) {
	t := NewUser()

	deser, err := compiler.CompileSchemaBytes([]byte(schema), []byte(t.Schema()))
	if err != nil {
		return t, err
	}

	err = vm.Eval(r, deser, &t)
	return t, err
}

func writeUser(r User, w io.Writer) error {
	var err error
	err = vm.WriteString(r.Id, w)
	if err != nil {
		return err
	}
	err = vm.WriteString(r.Name, w)
	if err != nil {
		return err
	}
	err = vm.WriteString(r.Username, w)
	if err != nil {
		return err
	}
	err = vm.WriteString(r.Created_at, w)
	if err != nil {
		return err
	}
	err = vm.WriteLong(r.Followers, w)
	if err != nil {
		return err
	}
	return err
}

func (r User) Serialize(w io.Writer) error {
	return writeUser(r, w)
}

func (r User) Schema() string {
	return "{\"fields\":[{\"name\":\"id\",\"type\":\"string\"},{\"name\":\"name\",\"type\":\"string\"},{\"name\":\"username\",\"type\":\"string\"},{\"name\":\"created_at\",\"type\":\"string\"},{\"name\":\"followers\",\"type\":\"long\"}],\"name\":\"User\",\"type\":\"record\"}"
}

func (r User) SchemaName() string {
	return "User"
}

func (_ User) SetBoolean(v bool)    { panic("Unsupported operation") }
func (_ User) SetInt(v int32)       { panic("Unsupported operation") }
func (_ User) SetLong(v int64)      { panic("Unsupported operation") }
func (_ User) SetFloat(v float32)   { panic("Unsupported operation") }
func (_ User) SetDouble(v float64)  { panic("Unsupported operation") }
func (_ User) SetBytes(v []byte)    { panic("Unsupported operation") }
func (_ User) SetString(v string)   { panic("Unsupported operation") }
func (_ User) SetUnionElem(v int64) { panic("Unsupported operation") }

func (r *User) Get(i int) types.Field {
	switch i {
	case 0:
		w := types.String{Target: &r.Id}

		return w

	case 1:
		w := types.String{Target: &r.Name}

		return w

	case 2:
		w := types.String{Target: &r.Username}

		return w

	case 3:
		w := types.String{Target: &r.Created_at}

		return w

	case 4:
		w := types.Long{Target: &r.Followers}

		return w

	}
	panic("Unknown field index")
}

func (r *User) SetDefault(i int) {
	switch i {
	}
	panic("Unknown field index")
}

func (r *User) NullField(i int) {
	switch i {
	}
	panic("Not a nullable field index")
}

func (_ User) AppendMap(key string) types.Field { panic("Unsupported operation") }
func (_ User) AppendArray() types.Field         { panic("Unsupported operation") }
func (_ User) HintSize(int)                     { panic("Unsupported operation") }
func (_ User) Finalize()                        {}

func (_ User) AvroCRC64Fingerprint() []byte {
	return []byte(UserAvroCRC64Fingerprint)
}

func (r User) MarshalJSON() ([]byte, error) {
	var err error
	output := make(map[string]json.RawMessage)
	output["id"], err = json.Marshal(r.Id)
	if err != nil {
		return nil, err
	}
	output["name"], err = json.Marshal(r.Name)
	if err != nil {
		return nil, err
	}
	output["username"], err = json.Marshal(r.Username)
	if err != nil {
		return nil, err
	}
	output["created_at"], err = json.Marshal(r.Created_at)
	if err != nil {
		return nil, err
	}
	output["followers"], err = json.Marshal(r.Followers)
	if err != nil {
		return nil, err
	}
	return json.Marshal(output)
}

func (r *User) UnmarshalJSON(data []byte) error {
	var fields map[string]json.RawMessage
	if err := json.Unmarshal(data, &fields); err != nil {
		return err
	}

	var val json.RawMessage
	val = func() json.RawMessage {
		if v, ok := fields["id"]; ok {
			return v
		}
		return nil
	}()

	if val != nil {
		if err := json.Unmarshal([]byte(val), &r.Id); err != nil {
			return err
		}
	} else {
		return fmt.Errorf("no value specified for id")
	}
	val = func() json.RawMessage {
		if v, ok := fields["name"]; ok {
			return v
		}
		return nil
	}()

	if val != nil {
		if err := json.Unmarshal([]byte(val), &r.Name); err != nil {
			return err
		}
	} else {
		return fmt.Errorf("no value specified for name")
	}
	val = func() json.RawMessage {
		if v, ok := fields["username"]; ok {
			return v
		}
		return nil
	}()

	if val != nil {
		if err := json.Unmarshal([]byte(val), &r.Username); err != nil {
			return err
		}
	} else {
		return fmt.Errorf("no value specified for username")
	}
	val = func() json.RawMessage {
		if v, ok := fields["created_at"]; ok {
			return v
		}
		return nil
	}()

	if val != nil {
		if err := json.Unmarshal([]byte(val), &r.Created_at); err != nil {
			return err
		}
	} else {
		return fmt.Errorf("no value specified for created_at")
	}
	val = func() json.RawMessage {
		if v, ok := fields["followers"]; ok {
			return v
		}
		return nil
	}()

	if val != nil {
		if err := json.Unmarshal([]byte(val), &r.Followers); err != nil {
			return err
		}
	} else {
		return fmt.Errorf("no value specified for followers")
	}
	return nil
}
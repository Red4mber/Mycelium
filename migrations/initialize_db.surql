THROW "DON'T RUN ME PLEASE";
-- =========================================================
-- ========= ALL IN ONE SURREALDB RE-INITIALIZER ===========
-- =========================================================


-- delete relations
DELETE controls; 
DELETE targets;

-- delete tables
DELETE operator, host, agent;

-- delete system accounts
REMOVE USER owner_account ON NAMESPACE;
REMOVE USER editor_account ON NAMESPACE;
REMOVE USER viewer_account ON NAMESPACE;

-- delete namespace and database
REMOVE NAMESPACE IF EXISTS mycelium_ns;
REMOVE DATABASE IF EXISTS mycelium_db;

-- =========================================================
-- =========================================================
-- =========================================================

DEFINE DATABASE IF NOT EXISTS mycelium_db;
DEFINE NAMESPACE IF NOT EXISTS mycelium_ns; --10

USE NS mycelium_ns DB mycelium_db;

-------------------------------------
    --- Adding database users ---
-------------------------------------

DEFINE USER IF NOT EXISTS owner_account -- 'Sulfur0-Everyone-Tweak'
    ON NAMESPACE PASSHASH '$argon2id$v=19$m=19456,t=2,p=1$4ypnx/9D/Pp4DcaqyAK/qQ$slsYiKNFLVqWO8ltEm45Ha9hqTSL7ZCKG/5cHE1pxOA'
    ROLES OWNER COMMENT "High privilege system account.";
DEFINE USER IF NOT EXISTS editor_account -- 'Sleet9-Implosive-Occupier'
    ON NAMESPACE PASSHASH '$argon2id$v=19$m=19456,t=2,p=1$xyJ8pE50MokRiO1jFDGJdQ$5CyOjZ8KPpf6vNEm0vKZhnxzpHUCVq5UfO88+mkckE0'
    ROLES EDITOR COMMENT "Medium privilege system account.";
DEFINE USER IF NOT EXISTS viewer_account -- 'Mounted-Drivable8-Giggling'
    ON NAMESPACE PASSHASH '$argon2id$v=19$m=19456,t=2,p=1$BZYVIa5CeyWxoTsYL4LK7g$il7nhkrqgjYJh2pB8FQ4vXHtB3a5RIRSvGlct5n8vjU'
    ROLES VIEWER COMMENT "Low privilege system account.";

------------------------------------------------
            --- CREATING TABLES ---
------------------------------------------------

------------------------------
    --- Operator table ---
------------------------------

USE NS mycelium_ns DB mycelium_db;
DEFINE TABLE operator SCHEMAFULL PERMISSIONS FOR select, create, update, delete WHERE user = $auth.id OR $auth.admin = true;
DEFINE FIELD name
    ON operator
    TYPE string;
DEFINE FIELD email
    ON operator
    TYPE string
    ASSERT string::is::email($value);
DEFINE FIELD pass
    ON operator
    TYPE string
    PERMISSIONS NONE;
DEFINE FIELD admin
    ON operator
    TYPE bool;
DEFINE FIELD enabled
    ON operator
    TYPE bool;

DEFINE INDEX email ON operator FIELDS email UNIQUE;

---
DEFINE ACCESS operator ON DATABASE TYPE RECORD 
	SIGNUP ( CREATE operator SET email = $email, pass = crypto::argon2::generate($pass) )
	SIGNIN ( SELECT * FROM operator WHERE email = $email AND crypto::argon2::compare(pass, $pass) )
    AUTHENTICATE {
        IF type::thing("token", $token.jti).revoked = true { THROW "This token has been revoked"; };
        INSERT INTO token { id: $token.jti, exp: $token.exp, revoked: false };
        CREATE audit_log CONTENT { 
            token: $token.jti, 
            time: time::now(), 
            account: $auth.id, 
            description: "New Operator token created." 
        };
        RETURN $auth;
    }

WITH JWT ALGORITHM ES384 KEY "-----BEGIN PUBLIC KEY-----
NfRIjofIo... -- < Keys cropped for security concerns
-- I rotate them regularly anyways but still rather avoid commiting stuff like that
.....-- Generate your own keys with the commands in the readme :)
.....JJzKYbNBnopAGYgKRmOGeS2
-----END PUBLIC KEY-----"
WITH ISSUER ALGORITHM ES384 KEY "-----BEGIN PRIVATE KEY-----
MIG2AgE...
/*


*/
...EiOh8igknMphs0GeikAZiApGY4Z5LY=
-----END PRIVATE KEY-----"
DURATION FOR TOKEN 15m, FOR SESSION 12h;



----------------------------
    --- Agents table ---
----------------------------

USE NS mycelium_ns DB mycelium_db;
DEFINE TABLE agent SCHEMALESS;
DEFINE FIELD time
    ON TABLE agent
    TYPE object
    DEFAULT {};
DEFINE FIELD time.created_at
    ON TABLE agent
    TYPE datetime
    VALUE $before OR time::now()
    DEFAULT time::now();
DEFINE FIELD time.updated_at
    ON TABLE agent
    TYPE datetime
    VALUE time::now()
    DEFAULT time::now();
DEFINE FIELD key
    ON TABLE agent
    TYPE String
    DEFAULT crypto::sha512(rand::string());
DEFINE FIELD host
    ON TABLE agent
    TYPE Record<host>;

DEFINE ACCESS agent ON DATABASE TYPE RECORD
	SIGNUP {
        IF $access = "operator" {
            CREATE operator CONTENT {
                id: rand::uuid::v7(),
                key: crypto::sha512(rand::string())
            }
        }
    }
	SIGNIN {
        IF !$access = "operator" { THROW "Please login." };
        SELECT * FROM agent WHERE id = $email;
    }
    AUTHENTICATE {
        IF type::thing("token", $token.jti).revoked = true {
            THROW "This token has been revoked";
        };
        INSERT INTO token {
            id: $token.jti,
            exp: $token.exp,
            revoked: false
        };
        CREATE audit_log CONTENT {
            token: $token.jti,
            time: time::now(),
            account: $auth.id,
            description: "New 'Agent' token created."
        };
        RETURN $auth;
    }
WITH JWT ALGORITHM ES384 KEY "-----BEGIN PUBLIC KEY-----
NfRIjofIo...
.....
.....-- Generate your own keys with the commands just below
.....JJzKYbNBnopAGYgKRmOGeS2
-----END PUBLIC KEY-----"
WITH ISSUER ALGORITHM ES384 KEY "-----BEGIN PRIVATE KEY-----
MIG2AgE...
-- openssl ecparam -name secp384r1 -genkey -noout -out sec1_ec_p384_private.pem
-- openssl pkcs8 -topk8 -nocrypt -in sec1_ec_p384_private.pem -out ec_p384_private.pem
-- rm sec1_ec_p384_private.pem
-- openssl ec -in ec_p384_private.pem -pubout -out ec_p384_public.pem
...EiOh8igknMphs0GeikAZiApGY4Z5LY=
-----END PRIVATE KEY-----"
    DURATION FOR TOKEN 15m, FOR SESSION 1h
;

----------------------------
    --- Hosts table ---
----------------------------

DEFINE TABLE host
    SCHEMALESS
    PERMISSIONS NONE;
DEFINE FIELD hostname
    ON TABLE host
    TYPE string;
DEFINE FIELD os
    ON TABLE host
    TYPE Object
    DEFAULT {};
DEFINE FIELD os.family
    ON TABLE host
    TYPE string;
DEFINE FIELD os.version
    ON TABLE host
    FLEXIBLE;
DEFINE FIELD arch
    ON TABLE host
    TYPE string
    ASSERT $value INSIDE ["I386", "AMD64", "ARM64", "ARM", "PowerPC", "MIPS", "Unknown"];

-- Add host reference to agent table
DEFINE FIELD host
    ON TABLE agent
    TYPE Option<Record<host>>;

----------------------------
    --- Files table ---
----------------------------

DEFINE TABLE file
    SCHEMALESS
    PERMISSIONS NONE;
DEFINE FIELD from_host
    ON TABLE file
    TYPE record<host>;
DEFINE FIELD filename
    ON TABLE file
    TYPE string;
DEFINE FIELD filepath
    ON TABLE file
    TYPE string;
DEFINE FIELD time.created_at
    ON TABLE file
    TYPE datetime
    VALUE $before OR time::now()
    DEFAULT time::now();
DEFINE FIELD time.updated_at
    ON TABLE file
    TYPE datetime
    VALUE time::now()
    DEFAULT time::now();

-- -- -- -- FIXME & Create events to trace stuff like this
-- FILE UPLOAD EVENT
-- DEFINE EVENT file_upload ON TABLE file WHEN $event = "CREATE" THEN (
--     CREATE audit_log CONTENT {
--             token: $token.jti,
--             time: time::now(),
--             account: $auth.id,
--             description: "File upload"
--         }
--     );

------------------------------------------------
            --- CREATING RELATIONS ---
------------------------------------------------

-- => Operator->Control->Agent
DEFINE TABLE control;
DEFINE FIELD in ON TABLE control TYPE record<operator>;
DEFINE FIELD out ON TABLE control TYPE record<agent>;
DEFINE INDEX unique_rel ON TABLE control COLUMNS in, out UNIQUE;

-- => Agent->Target->Host
DEFINE TABLE target;
DEFINE FIELD in ON TABLE target TYPE record<agent>;
DEFINE FIELD out ON TABLE target TYPE record<host>;
DEFINE INDEX unique_rel ON TABLE target COLUMNS in, out UNIQUE;

-- ==============================================================
-- ==============================================================
-- ==============================================================


CREATE operator:john CONTENT {
    email: "john.doe@example.com",
    name: 'John "Big D" Doe',
    pass: crypto::argon2::generate("V3ry_S3cur3"),
    admin: true,
    enabled: true
};
CREATE operator:jane CONTENT {
    email: "jane.doe@example.com",
    name: 'Jane "JayJay" Doe',
    pass: crypto::argon2::generate("Passw0rd123"),
    admin: false,
    enabled: true
};

CREATE agent:dummy  CONTENT { host: host:`0190f63b-db0a-795b-b277-cff60883387a` };
CREATE agent:dummy2 CONTENT { host: host:`0190f63b-db0a-795b-b277-cff60883387a` };
CREATE agent:dummy3 CONTENT { host: host:`0190f63b-db0a-795b-b277-cff60883387a` };

RELATE operator:john->control->agent:dummy;
RELATE operator:john->control->agent:dummy2;

RELATE operator:jane->control->agent:dummy3;

CREATE host:`0190f63b-db0a-795b-b277-cff60883387a` CONTENT {
    hostname: 'Desktop-F4K3PC',
    users: ["Administrator", "Harry Targetson"],
    host_id: u"0190f63b-db0a-795b-b277-cff60883387a",
    os: {
        family: "Windows",
        version: "Win11 22H2"
    },
    arch: "AMD64"
};

RELATE agent:dummy->target->host:desktop1;

CREATE file CONTENT {
    from_host: host:desktop1,
    filename: "fake_data.txt",
    filepath: "/loot/0190f63b-db0a-795b-b277-cff60883387a/fake_data.txt",
};

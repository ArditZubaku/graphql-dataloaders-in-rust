run:
	@cargo run --quiet

get_user:
	@curl -s -X POST http://localhost:3000/graphql \
	  -H "Content-Type: application/json" \
	  -d '{ \
	    "query": "query test($$id: ID!) { getUser(id: $$id) { id name } }", \
	    "variables": { "id": "1" } \
	  }' | jq

get_users:
	@curl -s -X POST http://localhost:3000/graphql \
	  -H "Content-Type: application/json" \
	  -d '{ \
	    "query": "query test2 { getUsers { id name org { id title } } }", \
	    "variables": { "id": "1" } \
	  }' | jq
